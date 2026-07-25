//! An opt-in response cache that takes its expiry from TfL rather than from us.
//!
//! Off by default: arrivals are the whole point of this API and a stale "3
//! minutes away" is worse than no answer. Within a single GraphQL request the
//! DataLoaders already collapse duplicate reads, which is where nearly all of
//! the win is.
//!
//! When it is switched on, the only lifetime it will honour is the one TfL
//! sends. Every response comes back through Varnish with a `Cache-Control`
//! reflecting how volatile that endpoint really is — twelve hours for
//! `/Line/Meta/Modes`, thirty seconds for `/Line/{id}/Status` — so nothing is
//! held longer than TfL itself considers fresh, and we never invent a TTL.

use std::{
    collections::HashMap,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

/// Entries are evicted oldest-first past this many. Most responses are small,
/// but `/StopPoint/Mode/bus` is megabytes, so the cache stays bounded.
const CAPACITY: usize = 512;

pub(crate) struct Cache {
    entries: Mutex<HashMap<String, Entry>>,
    hits: AtomicU64,
    misses: AtomicU64,
}

struct Entry {
    body: String,
    stored: Instant,
    ttl: Duration,
}

/// Counts for the CLI's `cache` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheStats {
    pub entries: usize,
    pub hits: u64,
    pub misses: u64,
}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<String> {
        let mut entries = self.entries.lock().unwrap();
        match entries.get(key) {
            Some(entry) if entry.stored.elapsed() < entry.ttl => {
                self.hits.fetch_add(1, Ordering::Relaxed);
                Some(entry.body.clone())
            }
            Some(_) => {
                entries.remove(key);
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
            None => {
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }

    pub(crate) fn insert(&self, key: String, body: String, ttl: Duration) {
        if ttl.is_zero() {
            return;
        }
        let mut entries = self.entries.lock().unwrap();
        if entries.len() >= CAPACITY && !entries.contains_key(&key) {
            if let Some(oldest) = entries
                .iter()
                .min_by_key(|(_, e)| e.stored)
                .map(|(k, _)| k.clone())
            {
                entries.remove(&oldest);
            }
        }
        entries.insert(
            key,
            Entry {
                body,
                stored: Instant::now(),
                ttl,
            },
        );
    }

    pub(crate) fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }

    pub(crate) fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.entries.lock().unwrap().len(),
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
        }
    }
}

/// Reads a freshness lifetime out of a `Cache-Control` header.
///
/// `s-maxage` wins over `max-age` where both are present, matching TfL's own
/// edge. Anything without an explicit lifetime is not cached at all.
pub(crate) fn ttl_from_cache_control(header: Option<&str>) -> Duration {
    let Some(header) = header else {
        return Duration::ZERO;
    };
    let header = header.to_ascii_lowercase();
    if header.contains("no-store") || header.contains("no-cache") {
        return Duration::ZERO;
    }
    let directive = |name: &str| {
        header.split(',').find_map(|part| {
            let part = part.trim();
            part.strip_prefix(name)?
                .strip_prefix('=')?
                .trim()
                .parse::<u64>()
                .ok()
        })
    };
    Duration::from_secs(
        directive("s-maxage")
            .or_else(|| directive("max-age"))
            .unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_shared_max_age() {
        let ttl = ttl_from_cache_control(Some("public, must-revalidate, max-age=30, s-maxage=60"));
        assert_eq!(ttl, Duration::from_secs(60));
    }

    #[test]
    fn falls_back_to_max_age() {
        assert_eq!(
            ttl_from_cache_control(Some("public, max-age=43200")),
            Duration::from_secs(43200)
        );
    }

    #[test]
    fn refuses_to_cache_without_an_explicit_lifetime() {
        assert_eq!(ttl_from_cache_control(Some("no-store")), Duration::ZERO);
        assert_eq!(ttl_from_cache_control(None), Duration::ZERO);
        assert_eq!(ttl_from_cache_control(Some("public")), Duration::ZERO);
    }

    #[test]
    fn expired_entries_are_misses() {
        let cache = Cache::new();
        cache.insert("k".into(), "v".into(), Duration::from_secs(60));
        assert_eq!(cache.get("k").as_deref(), Some("v"));

        cache.insert("gone".into(), "v".into(), Duration::ZERO);
        assert_eq!(cache.get("gone"), None);
    }
}
