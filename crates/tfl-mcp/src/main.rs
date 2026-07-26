//! `tfl` — London transport from the command line, and as an MCP server.

use std::sync::Arc;

use clap::{CommandFactory, Parser, Subcommand};
use tfl_api_client::{Client, Config};
use tfl_mcp::{
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
    /// Serve MCP over HTTP as well, rather than only stdio.
    ///
    /// Takes an address; defaults to 127.0.0.1:8080. **This is the only flag
    /// that puts MCP on HTTP** — the web flags below bring up a listener for
    /// their own surfaces without exposing MCP on it.
    #[arg(
        long,
        value_name = "ADDR",
        num_args = 0..=1,
        default_missing_value = DEFAULT_ADDR,
        global = true
    )]
    http: Option<String>,
    /// Serve a GraphQL endpoint at /graphql.
    #[arg(long, global = true)]
    graphql: bool,
    /// Serve the GraphiQL IDE at /. Implies --graphql.
    #[arg(long, global = true)]
    graphiql: bool,
    /// Open GraphiQL in a browser. Implies --graphiql, and so --graphql.
    #[arg(long, global = true)]
    browser: bool,
}

/// Where the web surfaces listen when no address was given.
///
/// Loopback rather than all interfaces: a flag typed on a laptop should not
/// quietly publish to the network. Deployments pass `--http 0.0.0.0:8080`.
const DEFAULT_ADDR: &str = "127.0.0.1:8080";

impl ServeArgs {
    /// Resolves the flags, each implying whatever it needs.
    ///
    /// `--browser` is meant to be the whole thing in one word, so it turns on
    /// GraphiQL, which turns on GraphQL. None of them turn on MCP-over-HTTP:
    /// that is what `--http` is for, and someone poking at GraphiQL locally has
    /// not asked to expose an MCP endpoint.
    fn resolve(self) -> (String, HttpSurfaces) {
        let graphiql = self.graphiql || self.browser;
        let graphql = self.graphql || graphiql;
        (
            self.http
                .clone()
                .unwrap_or_else(|| DEFAULT_ADDR.to_string()),
            HttpSurfaces {
                mcp: self.http.is_some(),
                graphql,
                graphiql,
                browser: self.browser,
            },
        )
    }

    /// Whether anything at all should be served over HTTP.
    fn wants_http(&self) -> bool {
        self.http.is_some() || self.graphql || self.graphiql || self.browser
    }
}

#[derive(Subcommand)]
enum Commands {
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
                .unwrap_or_else(|_| "tfl_mcp=info,tfl_api_client=info".into()),
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

    // No subcommand means serve, which is the whole job.
    let Some(command) = cli.command else {
        // Checked before binding: a server that starts and then fails every
        // query is worse than one that refuses to start.
        verify_app_key(&app_key).await?;
        if !cli.serve.wants_http() {
            return mcp::run_server(app_key).await;
        }
        let (addr, surfaces) = cli.serve.resolve();
        return mcp::run_http_server(&addr, app_key, surfaces).await;
    };

    match command {
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
