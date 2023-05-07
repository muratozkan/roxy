use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use std::error::Error;

#[tokio::main]
async fn main() {
    roxy::init_logger(LevelFilter::Info);

    if let Err(e) = process_args().await {
        log::error!("{}", e)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    // Admin server management commands (internal, used to manage roxy server from `roxy` command)
    AdminServer(AdminServerArgs),
    // Start server
    Start {
        #[arg(short, long, default_value_t = 3128, value_parser=clap::value_parser!(u16).range(1..16683))]
        port: u16,
    },
    // Print server status.
    Status,
    // Stops the running roxy server, if running.
    Stop,
    // Restarts the server
}

#[derive(Debug, Args)]
struct AdminServerArgs {
    #[command(subcommand)]
    command: AdminServerCommand,
}

#[derive(Debug, Subcommand)]
enum AdminServerCommand {
    Start {
        #[arg(short, long, default_value_t = 3128, value_parser=clap::value_parser!(u16).range(1..16683))]
        port: u16,
    },
}

#[derive(Args, Debug)]
struct StartAdminServerArgs {
    #[arg(short, long, default_value_t = 3128, value_parser=clap::value_parser!(u16).range(1..16683))]
    port: u16,
}

async fn process_args() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    match &args.command {
        Command::AdminServer(ref args) => process_admin_server_args(args).await,
        Command::Start { port } => roxy::start(port.to_owned()).await,
        Command::Status => roxy::server_status().await,
        Command::Stop => roxy::stop().await,
    }
}

async fn process_admin_server_args(args: &AdminServerArgs) -> Result<(), Box<dyn Error>> {
    match args.command {
        AdminServerCommand::Start { port } => roxy::controller::start_server(port).await,
    }
}
