//! The transport half of the client: authentication, retries and error mapping.

use std::{sync::Arc, time::Duration};

use tokio::sync::Semaphore;

use crate::{
    Error, Result,
    cache::{Cache, CacheStats, ttl_from_cache_control},
};

const DEFAULT_BASE_URL: &str = "https://api.tfl.gov.uk";
const USER_AGENT: &str = concat!(
    "tfl-mcp/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/radiosilence/tfl-mcp)"
);

/// How TfL is reached.
#[derive(Debug, Clone)]
pub struct Config {
    /// The `app_key` from <https://api-portal.tfl.gov.uk>. Without one TfL
    /// allows 50 requests/minute; with one, 500.
    pub app_key: Option<String>,
    pub base_url: String,
    pub timeout: Duration,
    /// Retries on 5xx and on throttling. Does not apply to a rejected key.
    pub max_retries: u32,
    /// Ceiling on requests in flight at once.
    ///
    /// A GraphQL query can ask for something whose width is not obvious from
    /// its shape — every stop on every bus line is 676 lines, two levels deep —
    /// and each of those becomes a request. Without a ceiling one tool call
    /// opens hundreds of sockets and burns a minute's rate limit in a second.
    /// The GraphQL layer rejects the truly absurd queries, but this is the
    /// backstop that does not depend on anticipating the shape.
    pub max_concurrency: usize,
    /// Reuse *live* responses for as long as TfL's `Cache-Control` says they
    /// are fresh. Off by default so live data stays live.
    ///
    /// Reference data is cached regardless — see [`is_reference_data`].
    pub cache: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_key: None,
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 2,
            // TfL allows 500 requests/minute with a key and 50 without. Eight
            // at a time keeps a burst well inside either while still being far
            // faster than serial.
            max_concurrency: 8,
            cache: false,
        }
    }
}

pub struct Client {
    http: reqwest::Client,
    config: Config,
    cache: Cache,
    in_flight: Arc<Semaphore>,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        // TfL accepts `app_key` as a header as well as a query parameter, which
        // keeps the secret out of URLs and therefore out of logs.
        //
        // An *invalid* key is worse than none at all: TfL answers it with 429
        // where an anonymous caller would have got a 200. So a blank or
        // whitespace-only key — an unset environment variable, usually — is
        // dropped rather than sent.
        if let Some(key) = config
            .app_key
            .as_deref()
            .map(str::trim)
            .filter(|k| !k.is_empty())
            && let Ok(mut value) = reqwest::header::HeaderValue::from_str(key)
        {
            value.set_sensitive(true);
            headers.insert("app_key", value);
        }

        Ok(Self {
            http: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .timeout(config.timeout)
                .default_headers(headers)
                .build()?,
            in_flight: Arc::new(Semaphore::new(config.max_concurrency.max(1))),
            config,
            cache: Cache::new(),
        })
    }

    pub fn has_app_key(&self) -> bool {
        self.config
            .app_key
            .as_deref()
            .is_some_and(|k| !k.trim().is_empty())
    }

    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Verifies the configured key, if there is one.
    ///
    /// Worth calling at startup: a mistyped key otherwise shows up as
    /// throttling on the first real query, hours later and nowhere near the
    /// cause.
    pub async fn check_credentials(&self) -> Result<()> {
        if !self.has_app_key() {
            return Ok(());
        }
        self.get::<serde_json::Value>("/Line/Meta/Severity", &[])
            .await
            .map(|_| ())
    }

    /// Performs a GET and decodes the response.
    ///
    /// Every generated endpoint method funnels through here.
    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T> {
        // Reference data is cached whatever the configuration says, and live
        // data only when asked for. Deciding by *what the endpoint is* rather
        // than by its TTL removes the footgun: no setting can make an arrival
        // time stale, and the vocabulary endpoints a model reads before every
        // query stop costing a request each time.
        let cacheable = self.config.cache || is_reference_data(path);

        let key = cache_key(path, query);
        if cacheable && let Some(body) = self.cache.get(&key) {
            return decode(path, &body);
        }

        // Held for the whole request including retries, so a retry storm cannot
        // widen past the ceiling either. The semaphore is never closed, so
        // acquire only fails if it were.
        let _permit = self
            .in_flight
            .clone()
            .acquire_owned()
            .await
            .expect("the in-flight semaphore is never closed");

        let url = format!("{}{path}", self.config.base_url);
        // One line per upstream request, so the batching the DataLoaders do is
        // observable rather than assumed.
        tracing::debug!(path, "GET");
        let mut attempt = 0;
        loop {
            let response = self.http.get(&url).query(query).send().await?;
            let status = response.status();

            if status.is_success() {
                let ttl = ttl_from_cache_control(
                    response
                        .headers()
                        .get(reqwest::header::CACHE_CONTROL)
                        .and_then(|v| v.to_str().ok()),
                );
                let body = response.text().await?;
                if cacheable {
                    self.cache.insert(key, body.clone(), ttl);
                }
                return decode(path, &body);
            }

            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            let body = response.text().await.unwrap_or_default();

            // 300 Multiple Choices is the journey planner asking which
            // "Victoria" you meant. Not an error, and not retryable.
            if status == reqwest::StatusCode::MULTIPLE_CHOICES {
                return Err(Error::Ambiguous {
                    path: path.to_string(),
                    body,
                });
            }

            // TfL reports a bad key as 429 with a distinctive body. Retrying
            // that just burns the rate limit on a request that can never work.
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS
                && body.to_ascii_lowercase().contains("invalid app_key")
            {
                return Err(Error::InvalidAppKey);
            }

            let retryable =
                status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS;
            if !retryable || attempt >= self.config.max_retries {
                return Err(match status {
                    reqwest::StatusCode::TOO_MANY_REQUESTS => Error::RateLimited { retry_after },
                    _ => Error::Status {
                        status,
                        path: path.to_string(),
                        body: body.chars().take(500).collect(),
                    },
                });
            }

            let backoff = retry_after
                .map(Duration::from_secs)
                .unwrap_or_else(|| Duration::from_millis(250 * 2u64.pow(attempt)));
            tracing::debug!(%status, attempt, ?backoff, path, "retrying TfL request");
            tokio::time::sleep(backoff).await;
            attempt += 1;
        }
    }
}

impl Client {
    /// Performs a GET for an endpoint the spec says returns a list.
    ///
    /// `/StopPoint/{ids}` given exactly one id answers with a bare object
    /// instead of a one-element array, so decoding straight into a `Vec` fails
    /// on precisely the request a DataLoader makes when it has one key left
    /// over. Every list endpoint goes through here rather than only that one:
    /// the same trap is a plausible response to any of them.
    pub async fn get_list<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<Vec<T>> {
        let raw: serde_json::Value = self.get(path, query).await?;

        // Tried in order rather than through an untagged enum. `#[serde(
        // untagged)]` reports only "data did not match any variant" and throws
        // away the real error, so a genuine schema mismatch becomes
        // undiagnosable — which field, which line, all gone. Decoding the list
        // shape first and surfacing *its* error keeps the message that says
        // what actually went wrong.
        match serde_json::from_value::<Vec<T>>(raw.clone()) {
            Ok(items) => Ok(items),
            Err(list_error) => match serde_json::from_value::<T>(raw) {
                Ok(single) => Ok(vec![single]),
                Err(_) => Err(Error::Decode {
                    path: path.to_string(),
                    source: list_error,
                }),
            },
        }
    }
}

/// Whether a path returns vocabulary rather than observations.
///
/// TfL's `Meta` endpoints are the words the rest of the API expects — mode
/// names, stop types, severity codes, service types. They change when TfL
/// launches a line, and TfL itself marks them cacheable for twelve hours,
/// where line status is thirty seconds. A model reads several of them before
/// it can write its first sensible query, so re-fetching them per request is
/// pure waste with no freshness argument against it.
///
/// The list is exact rather than a substring match: `/Line/Mode/{modes}/Status`
/// contains "Mode" and is emphatically live.
fn is_reference_data(path: &str) -> bool {
    const REFERENCE_PREFIXES: &[&str] = &[
        "/Line/Meta/",
        "/StopPoint/Meta/",
        "/Road/Meta/",
        "/Place/Meta/",
        "/Search/Meta/",
        "/Journey/Meta/",
    ];
    REFERENCE_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

fn decode<T: serde::de::DeserializeOwned>(path: &str, body: &str) -> Result<T> {
    serde_json::from_str(body).map_err(|source| Error::Decode {
        path: path.to_string(),
        source,
    })
}

fn cache_key(path: &str, query: &[(&str, String)]) -> String {
    let mut key = String::from(path);
    for (name, value) in query {
        key.push(if key.contains('?') { '&' } else { '?' });
        key.push_str(name);
        key.push('=');
        key.push_str(value);
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_keys_are_not_sent() {
        // A blank key would earn a 429 where sending nothing earns a 200.
        for key in [None, Some(""), Some("   ")] {
            let client = Client::new(Config {
                app_key: key.map(str::to_string),
                ..Default::default()
            })
            .unwrap();
            assert!(!client.has_app_key(), "{key:?} should not count as a key");
        }
    }

    #[test]
    fn only_vocabulary_is_cached_unconditionally() {
        // Vocabulary: changes when TfL launches a line.
        for path in [
            "/Line/Meta/Modes",
            "/Line/Meta/Severity",
            "/StopPoint/Meta/StopTypes",
            "/Road/Meta/Severities",
        ] {
            assert!(is_reference_data(path), "{path} should be cached");
        }

        // Observations: caching any of these would serve a departure that has
        // already left. `/Line/Mode/tube/Status` is the trap — it contains
        // "Mode" but is live, which is why the check is a prefix and not a
        // substring.
        for path in [
            "/Line/Mode/tube/Status",
            "/StopPoint/940GZZLUKSX/Arrivals",
            "/Line/victoria/Status",
            "/Road/all/Disruption",
            "/BikePoint",
            "/AirQuality",
        ] {
            assert!(!is_reference_data(path), "{path} must not be cached");
        }
    }

    #[test]
    fn query_is_part_of_the_cache_key() {
        assert_eq!(cache_key("/Line/victoria", &[]), "/Line/victoria");
        assert_eq!(
            cache_key("/Line/victoria", &[("detail", "true".into())]),
            "/Line/victoria?detail=true"
        );
    }
}
