//! `tfl` — London transport from the command line, and as an MCP server.

use std::sync::Arc;

use clap::{CommandFactory, Parser, Subcommand};
use tfl_api_client::{Client, Config};
use tfl_cli::{
    app_key_from_env, cache_from_env,
    mcp::{self, HttpSurfaces, graphql},
    output::Output,
};

/// Runs the MCP server unless a subcommand says otherwise.
///
/// Serving is the whole job — the subcommands exist to inspect and debug it,
/// not the other way round.
#[derive(Parser)]
#[command(name = "tfl", version, about = "London transport, as GraphQL and MCP")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    serve: ServeArgs,
}

#[derive(clap::Args, Clone)]
struct ServeArgs {
    /// Serve over HTTP on this address instead of stdio, e.g. `0.0.0.0:8080`.
    #[arg(long, value_name = "ADDR", global = true)]
    http: Option<String>,
    /// Also serve a plain GraphQL endpoint at /graphql.
    #[arg(long, global = true)]
    graphql: bool,
    /// Also serve the GraphiQL IDE at /, and open a browser.
    #[arg(long, global = true)]
    graphiql: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Deprecated: running the server is the default, so `tfl --http …` does
    /// the same thing.
    ///
    /// Kept and hidden so a deployment pinning an image tag independently of
    /// its arguments cannot break on a version bump alone.
    #[command(hide = true)]
    Mcp,
    /// Run a GraphQL query. Use `tfl schema` to see what is available.
    Query {
        /// The query. Reads stdin if omitted.
        query: Option<String>,
    },
    /// Print the GraphQL schema as SDL.
    Schema,
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

    // No subcommand, or the deprecated `mcp` one: serve.
    if matches!(cli.command, None | Some(Commands::Mcp)) {
        // Checked before binding: a server that starts and then fails every
        // query is worse than one that refuses to start.
        verify_app_key(&app_key).await?;
        let ServeArgs {
            http,
            graphql: serve_graphql,
            graphiql,
        } = cli.serve;
        return match http {
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
        };
    }

    match cli.command.expect("serving already returned") {
        Commands::Mcp => unreachable!("handled above"),
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

        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "tfl", &mut std::io::stdout());
            Ok(())
        }
    }
}

fn client(app_key: Option<String>) -> anyhow::Result<Arc<Client>> {
    Ok(Arc::new(Client::new(Config {
        app_key,
        cache: cache_from_env(),
        ..Default::default()
    })?))
}

/// Fails fast on a key TfL will not accept.
///
/// Without this a mistyped key surfaces as a 429 on the first real query, which
/// is indistinguishable from ordinary throttling and turns up hours later,
/// nowhere near the cause.
async fn verify_app_key(app_key: &Option<String>) -> anyhow::Result<()> {
    if app_key.is_none() {
        return Ok(());
    }
    client(app_key.clone())?
        .check_credentials()
        .await
        .map_err(|e| anyhow::anyhow!("{e}. Unset TFL_APP_KEY to use TfL anonymously."))
}
