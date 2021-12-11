use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AccountRecoverySettingType {
    recovery_mechanism: Option<Vec<super::RecoveryOptionType>>,
}
