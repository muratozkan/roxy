//! # proxy
//!
//! this module contains the components that power the
//! roxy web proxy
//!

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::task::JoinHandle;

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

pub async fn start(args: Arguments) -> Result<JoinHandle<()>, Error> {
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(reverse_proxy)) });

    let server = Server::try_bind(&addr)
        .map_err(|e| Error::CouldNotStart(e.to_string()))?
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    let handle = tokio::spawn(async move {
        log::info!("proxy started and listening @ {}", addr.to_string());
        // Await the `server` receiving the signal...
        if let Err(e) = server.await {
            log::info!("Server exiting with error: {}", e)
        }
    });

    Ok(handle)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.err();
}

async fn reverse_proxy(req: Request<Body>) -> Result<Response<Body>, Error> {
    let https = HttpsConnector::new();

    let mut builder = Request::builder().uri("https://httpbin.org/anything");
    for (key, value) in req.headers().iter() {
        builder = builder.header(key.as_str(), value.as_bytes());
    }
    let body = req.into_body();
    let proxy_req = builder
        .body(body)
        .map_err(|e| Error::BadExit(e.to_string()))?;

    log::info!("Proxy request: {:?}", &proxy_req);
    let proxy_resp = hyper::Client::builder()
        .build(https)
        .request(proxy_req)
        .await
        .unwrap();
    log::info!("Response: {:?}", &proxy_resp);
    Ok(proxy_resp)
}
