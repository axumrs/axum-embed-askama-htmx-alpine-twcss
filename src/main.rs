use aeahat::handler;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .thread_name(env!("CARGO_CRATE_NAME"))
        .enable_all()
        .build()?;
    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9527").await?;
    let app = router_init();
    axum::serve(listener, app).await?;
    Ok(())
}

fn router_init() -> Router {
    Router::new()
        .merge(web_router_init())
        .fallback(handler::static_file::static_handler)
}
fn web_router_init() -> Router {
    Router::new().route("/", get(handler::web::index))
}
