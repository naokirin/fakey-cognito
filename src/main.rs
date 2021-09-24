use fakey_cognito::*;

const DEFAULT_LOG_LEVEL: &str = "info";
const LOG_LEVEL_KEY: &str = "RUST_LOG";

fn setup_logger() {
    let log_level = opts::get_opt_log_level()
        .or_else(|| std::env::var(LOG_LEVEL_KEY).ok())
        .or_else(|| Some(DEFAULT_LOG_LEVEL.to_string()))
        .unwrap();
    std::env::set_var(LOG_LEVEL_KEY, log_level);
    pretty_env_logger::init();
}

#[tokio::main]
async fn main() {
    opts::init_opt().await;
    setup_logger();
    let templates_opt = opts::get_opt_templates();
    tokio::join!(
        user_pools::init_config(opts::get_opt_config()),
        templates::init_template(templates_opt.map(String::as_str)),
        templates::init_default_template()
    );

    let port = opts::get_opt_port().unwrap_or(8080);
    warp::serve(routes::user_pools_routes())
        .run(([0, 0, 0, 0], port))
        .await;
}
