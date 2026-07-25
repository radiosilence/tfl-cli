//! `tfl` — London transport from the command line, and as an MCP server.

use std::sync::Arc;

use clap::{CommandFactory, Parser, Subcommand};
use tfl_api_client::{Client, Config};
use tfl_cli::{
    app_key_from_env,
    mcp::{self, HttpSurfaces, graphql},
    output::Output,
};

#[derive(Parser)]
#[command(name = "tfl", version, about = "London transport, as GraphQL and MCP")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as an MCP server.
    Mcp {
        /// Serve over HTTP on this address instead of stdio, e.g. `0.0.0.0:8080`.
        #[arg(long, value_name = "ADDR")]
        http: Option<String>,
        /// Also serve a plain GraphQL endpoint at /graphql.
        #[arg(long)]
        graphql: bool,
        /// Also serve the GraphiQL IDE at /, and open a browser.
        #[arg(long)]
        graphiql: bool,
    },
    /// Run a GraphQL query. Use `tfl schema` to see what is available.
    Query {
        /// The query. Reads stdin if omitted.
        query: Option<String>,
    },
    /// Print the GraphQL schema as SDL.
    Schema,
    /// Live arrivals at a stop, soonest first.
    Arrivals {
        /// NaPTAN id, e.g. `940GZZLUKSX`, or a station name to search for.
        stop: String,
        /// How many to show.
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Status of every line on a mode.
    Status {
        /// Modes to report on.
        #[arg(long, default_value = "tube,dlr,overground,elizabeth-line")]
        modes: String,
        /// Only show lines that are not running a good service.
        #[arg(long)]
        disrupted: bool,
    },
    /// Find stops by name.
    Search {
        /// A station or stop name, e.g. `Kings Cross`.
        query: String,
    },
    /// Generate a shell completion script.
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tfl_cli=info,tfl_api_client=info".into()),
        )
        // stdout carries the JSON envelope, so logs go to stderr or they would
        // make it unparseable.
        .with_writer(std::io::stderr)
        .init();

    if let Err(e) = run().await {
        Output::<()>::error(e.to_string()).print();
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app_key = app_key_from_env();

    match cli.command {
        Commands::Mcp {
            http,
            graphql: serve_graphql,
            graphiql,
        } => match http {
            Some(addr) => {
                mcp::run_http_server(
                    &addr,
                    app_key,
                    HttpSurfaces {
                        mcp: true,
                        graphql: serve_graphql || graphiql,
                        graphiql,
                        browser: graphiql,
                    },
                )
                .await
            }
            None => mcp::run_server(app_key).await,
        },

        Commands::Schema => {
            println!("{}", graphql::sdl());
            Ok(())
        }

        Commands::Query { query } => {
            let query = match query {
                Some(q) => q,
                None => std::io::read_to_string(std::io::stdin())?,
            };
            let response = graphql::schema()
                .execute(graphql::request(&query, client(app_key)?))
                .await;
            println!("{}", serde_json::to_string_pretty(&response)?);
            // A GraphQL error is a failed command, not a successful empty one.
            if response.is_err() {
                std::process::exit(1);
            }
            Ok(())
        }

        // The remaining commands are thin wrappers over the same graph the MCP
        // server exposes, so there is one implementation of every join.
        Commands::Arrivals { stop, limit } => {
            // A NaPTAN id is uppercase alphanumeric; anything else is a name to
            // search for, so `tfl arrivals "Kings Cross"` does what you mean.
            let looks_like_an_id = stop
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
            let entry = if looks_like_an_id {
                format!("stopPoint(id: {})", quote(&stop))
            } else {
                format!("searchStopPoints(query: {})", quote(&stop))
            };
            run_query(
                &format!(
                    "{{ {entry} {{ commonName \
                       arrivals(first: {limit}) {{ timeToStation towards platformName \
                       line {{ name }} }} }} }}"
                ),
                app_key,
            )
            .await
        }

        Commands::Status { modes, disrupted } => {
            let modes = modes
                .split(',')
                .map(|m| quote(m.trim()))
                .collect::<Vec<_>>()
                .join(", ");
            let field = if disrupted {
                "disruptedLines"
            } else {
                "linesByMode"
            };
            run_query(
                &format!(
                    "{{ {field}(modes: [{modes}]) {{ id name mode \
                       statuses {{ description reason }} }} }}"
                ),
                app_key,
            )
            .await
        }

        Commands::Search { query } => {
            run_query(
                &format!(
                    "{{ searchStopPoints(query: {}) {{ id commonName modes lat lon }} }}",
                    quote(&query)
                ),
                app_key,
            )
            .await
        }

        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "tfl", &mut std::io::stdout());
            Ok(())
        }
    }
}

/// Renders a value as a GraphQL string literal.
///
/// Station names contain apostrophes and quotes — `King's Cross`, `Shepherd's
/// Bush (Market)` — and an unescaped one would end the literal early and make
/// the rest of the query mean something else.
fn quote(value: &str) -> String {
    serde_json::Value::String(value.to_string()).to_string()
}

async fn run_query(query: &str, app_key: Option<String>) -> anyhow::Result<()> {
    let response = graphql::schema()
        .execute(graphql::request(query, client(app_key)?))
        .await;
    if response.is_err() {
        let errors: Vec<String> = response.errors.iter().map(|e| e.message.clone()).collect();
        anyhow::bail!("{}", errors.join("; "));
    }
    Output::ok(response.data.into_json()?).print();
    Ok(())
}

fn client(app_key: Option<String>) -> anyhow::Result<Arc<Client>> {
    Ok(Arc::new(Client::new(Config {
        app_key,
        ..Default::default()
    })?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn station_names_with_apostrophes_stay_inside_the_literal() {
        assert_eq!(quote("King's Cross"), "\"King's Cross\"");
        assert_eq!(quote("a\"b"), "\"a\\\"b\"");
    }
}
