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
        } => {
            run_node(listen_addr, listen_port, data_dir).await;
        }
        Commands::Version => {
            println!("Agee Chain v0.1.0");
            println!("Proof-of-Arcade native blockchain");
        }
    }
}

async fn run_node(listen_addr: String, listen_port: u16, data_dir: String) {
    let config = NodeConfig {
        chain_id: [0u8; 32],
        listen_addr: listen_addr.clone(),
        listen_port,
        data_dir,
    };

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
