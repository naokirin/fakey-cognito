use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AccountRecoverySettingType {
    #[validate]
    #[validate(length(min = 1, max = 2))]
    recovery_mechanism: Option<Vec<super::RecoveryOptionType>>,
}
