use std::path::PathBuf;
use structopt::StructOpt;
use tokio::sync::OnceCell;

pub static OPT: OnceCell<Opt> = OnceCell::const_new();

#[derive(StructOpt, Debug)]
#[structopt(name = "fakey-cognito", about = "Fake cognito api server.")]
pub struct Opt {
    /// Read specific config path
    #[structopt(long, name = "config_path", parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Read specific templates directory path
    #[structopt(long, name = "templates_path")]
    pub templates: Option<String>,

    /// Port number
    #[structopt(short, long, name = "port")]
    pub port: Option<u16>,

    /// Log level (debug, info, warn, error)
    #[structopt(long, name = "level")]
    pub log_level: Option<String>,
}

pub async fn init_opt() {
    OPT.get_or_init(|| async move { Opt::from_args() }).await;
}

pub fn get_opt_config() -> Option<&'static PathBuf> {
    OPT.get().unwrap().config.as_ref()
}

pub fn get_opt_templates() -> Option<&'static String> {
    OPT.get().unwrap().templates.as_ref()
}

pub fn get_opt_port() -> Option<u16> {
    OPT.get().unwrap().port
}

pub fn get_opt_log_level() -> Option<String> {
    OPT.get().unwrap().log_level.clone()
}
