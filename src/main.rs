mod common;
mod http;
mod routes;
mod user_pools;

const DEFAULT_LOG_LEVEL: &str = "debug";
const LOG_LEVEL_KEY: &str = "RUST_LOG";

fn setup_logger() {
    let log_level = std::env::var(LOG_LEVEL_KEY).unwrap_or(DEFAULT_LOG_LEVEL.to_string());
    std::env::set_var(LOG_LEVEL_KEY, log_level);
    pretty_env_logger::init();
}

#[tokio::main]
async fn main() {
    setup_logger();

    warp::serve(routes::user_pools_routes())
        .run(([127, 0, 0, 1], 8080))
        .await;
}
