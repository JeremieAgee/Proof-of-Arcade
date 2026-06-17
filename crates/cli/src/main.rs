use agee_node::{NodeConfig, NodeRuntime, create_router};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "agee-node")]
#[command(about = "Agee Arcade Coin native blockchain node")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the node server
    Run {
        #[arg(long, default_value = "127.0.0.1")]
        listen_addr: String,

        #[arg(long, default_value = "8080")]
        listen_port: u16,

        #[arg(long, default_value = "./data")]
        data_dir: String,

        #[arg(long, default_value = "agee-genesis.toml")]
        genesis_config: String,
    },
    /// Print version info
    Version,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            listen_addr,
            listen_port,
            data_dir,
            genesis_config,
        } => {
            run_node(listen_addr, listen_port, data_dir, genesis_config).await;
        }
        Commands::Version => {
            println!("Agee Chain v0.1.0");
            println!("Proof-of-Arcade native blockchain");
        }
    }
}

async fn run_node(listen_addr: String, listen_port: u16, data_dir: String, genesis_config: String) {
    let mut config = match NodeConfig::from_toml_file(&genesis_config) {
        Ok(cfg) => {
            tracing::info!("Loaded genesis config from {}", genesis_config);
            cfg
        }
        Err(e) => {
            tracing::warn!("Failed to load genesis config: {}. Using defaults.", e);
            NodeConfig::default()
        }
    };

    config.listen_addr = listen_addr.clone();
    config.listen_port = listen_port;
    config.data_dir = data_dir;

    let runtime = Arc::new(NodeRuntime::new(config));
    let router = create_router(runtime.clone());

    let addr = format!("{}:{}", listen_addr, listen_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}", addr));

    tracing::info!("Agee Node listening on {}", addr);
    tracing::info!("Chain info: GET /chain/info");
    tracing::info!("Balance: GET /balance/<account>");
    tracing::info!("Supply: GET /supply");
    tracing::info!("Claim floor: POST /tx/claim-floor");

    axum::serve(listener, router)
        .await
        .expect("Server error");
}
