//! # roxy
//!
//! roxy is a local development web proxy.
//! it enables users to configure routes, certificates, and plugins that can
//! process network traffic and forward it to another destintion on your development
//! machine
//!
mod controller;
pub mod logger;
mod proxy;

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start up the local development proxy
    Start(StartArgs),
    /// Add various entities to a running proxy
    Add { entity: Entity },
}

#[derive(Args, Debug)]
struct StartArgs {
    #[arg(short, long, default_value_t = 3128)]
    port: u16,
}

impl From<&StartArgs> for proxy::Arguments {
    fn from(value: &StartArgs) -> Self {
        Self { port: value.port }
    }
}

impl From<&StartArgs> for controller::server::ControlServerArgs {
    fn from(value: &StartArgs) -> Self {
        Self { port: value.port }
    }
}

#[derive(Clone, ValueEnum)]
enum Entity {
    /// Add a route to the proxy
    Route,
    /// Add a global plugin to the proxy (that will be executed on each route)
    Plugin,
    /// Add a certificate to the proxy
    Certificate,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();

    match &args.command {
        Command::Start(args) => {
            controller::server::start_server(args.into()).await?;
            proxy::start(args.into()).await?;
            Ok(())
        }
        Command::Add { entity } => match entity {
            Entity::Route => {
                log::info!("todo: implement add route");
                Ok(())
            }
            Entity::Plugin => {
                log::info!("todo: implement add plugin");
                Ok(())
            }
            Entity::Certificate => {
                log::info!("todo: implement add certificate");
                Ok(())
            }
        },
    }
}
