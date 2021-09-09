use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::OnceCell;

const DEFAULT_USER_POOLS_CONFIG_PATH: &str = "./user_pools.yml";
static CONFIG: OnceCell<Option<Config>> = OnceCell::const_new();

pub const CONFIG_STATUS_NAME: &str = "status_name";
pub const CONFIG_TEMPLATE_NAME: &str = "template";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub admin_add_user_to_group: Option<HashMap<String, String>>,
}

/// Initializes global config.
pub async fn init_config(path: Option<&PathBuf>) {
    CONFIG
        .get_or_init(|| async { read_config(path).ok() })
        .await;
}

fn read_config(path: Option<&PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
    let default_path = PathBuf::from(DEFAULT_USER_POOLS_CONFIG_PATH);
    let path = path.unwrap_or(&default_path);
    let s = std::fs::read_to_string(path)?;

    log::info!("read config: {}", path.display());
    let m: Config = serde_yaml::from_str(&s)?;
    Ok(m)
}

/// Returns config.
pub fn config() -> &'static Option<Config> {
    CONFIG.get().unwrap_or(&None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TEST_CONFIG: &str = "resources/test/test_valid_user_pools.yml";
    const INVALID_TEST_CONFIG: &str = "resources/test/test_invalid_user_pools.yml";

    #[test]
    fn success_to_read_valid_config() {
        let config = read_config(Some(&PathBuf::from(VALID_TEST_CONFIG)));
        assert!(config.is_ok());

        let admin_add_user_to_group = config.unwrap().admin_add_user_to_group;
        assert!(admin_add_user_to_group.is_some());
        assert_eq!(
            Some(&"InternalFailure".to_string()),
            admin_add_user_to_group.unwrap().get("status_name")
        );
    }

    #[test]
    fn error_to_read_invalid_config() {
        let config = read_config(Some(&PathBuf::from(INVALID_TEST_CONFIG)));
        assert!(config.is_err());
    }
}
