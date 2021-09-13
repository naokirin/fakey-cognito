use std::{collections::HashMap, path::PathBuf};
use tokio::sync::OnceCell;

pub type Config = HashMap<String, HashMap<String, String>>;

const DEFAULT_USER_POOLS_CONFIG_PATH: &str = "./user_pools.yml";
static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub const CONFIG_ERROR_TYPE: &str = "error_type";

pub fn get_config(action: &str, name: &String) -> Option<String> {
    super::config()
        .get(action)
        .unwrap_or(&HashMap::new())
        .get(name)
        .map(|c| c.clone())
}

/// Initializes global config.
pub async fn init_config(path: Option<&PathBuf>) {
    CONFIG
        .get_or_init(|| async {
            match read_config(path) {
                Ok(c) => c,
                Err(e) => {
                    log::error!("{}", e);
                    HashMap::new()
                }
            }
        })
        .await;
}

fn read_config(path: Option<&PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
    let default_path = PathBuf::from(DEFAULT_USER_POOLS_CONFIG_PATH);
    if path == None && !default_path.exists() {
        return Ok(Default::default());
    }
    let path = path.unwrap_or(&default_path);
    if !path.exists() {
        log::error!("No such file or directory: `{}`", path.display());
        return Ok(Default::default());
    }

    let s = std::fs::read_to_string(path)?;

    log::info!("read config: {}", path.display());
    let m: Config = serde_yaml::from_str(&s)?;
    Ok(m)
}

/// Returns config.
pub fn config() -> &'static Config {
    CONFIG.get().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const VALID_TEST_CONFIG: &str = "resources/test/test_valid_user_pools.yml";
    const INVALID_TEST_CONFIG: &str = "resources/test/test_invalid_user_pools.yml";

    #[test]
    fn success_to_read_valid_config() {
        let config = read_config(Some(&PathBuf::from(VALID_TEST_CONFIG)));
        assert!(config.is_ok());

        let admin_add_user_to_group = config.as_ref().unwrap().get("AdminAddUserToGroup");
        assert!(admin_add_user_to_group.is_some());
        assert_eq!(
            Some(&"InternalFailure".to_string()),
            admin_add_user_to_group.unwrap().get(CONFIG_ERROR_TYPE)
        );
    }

    #[test]
    fn error_to_read_invalid_config() {
        let config = read_config(Some(&PathBuf::from(INVALID_TEST_CONFIG)));
        assert!(config.is_err());
    }
}
