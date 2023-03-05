//! # proxy
//!
//! this module contains the components that power the
//! roxy web proxy
//!

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The proxy failed to start: {0}")]
    CouldNotStart(String),
    #[error("The proxy terminated unexpectedly: {0}")]
    BadExit(String),
}

pub struct Arguments {
    pub port: u16,
}

pub async fn start(args: Arguments) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(ping))
    });

    let server = match Server::try_bind(&addr) {
        Ok(server) => server,
        Err(e) => return Err(Error::CouldNotStart(e.to_string())),
    };

    let server = server
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    log::info!("proxy started and listening @ {}", addr.to_string());

    // Await the `server` receiving the signal...
    if let Err(e) = server.await {
        return Err(Error::BadExit(e.to_string()));
    }

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.err();
}

async fn ping(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("roxy is local dev proxy that rox!".into()))
}
