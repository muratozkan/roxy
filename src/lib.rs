//! # roxy
//!
//! roxy is a local development web proxy.
//! it enables users to configure routes, certificates, and plugins that can
//! process network traffic and forward it to another destintion on your development
//! machine
//!

use std::error::Error;

use log::LevelFilter;

pub mod controller;
mod logger;
mod proxy;

pub fn init_logger(level: LevelFilter) {
    logger::init(level);
}

pub async fn start(port: u16) -> Result<(), Box<dyn Error>> {
    log::info!("Starting roxy. Admin server: http://localhost:{}/", port);
    Ok(())
}

pub async fn server_status() -> Result<(), Box<dyn Error>> {
    log::info!("roxy status: stopped");
    Ok(())
}

pub async fn stop() -> Result<(), Box<dyn Error>> {
    log::info!("Stopping roxy admin server");
    Ok(())
}
