#[tokio::main]
async fn main() {
    roxy::logger::init();

    if let Err(e) = roxy::run().await {
        log::error!("{}", e)
    }
}
