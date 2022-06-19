use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceConfigurationType {
    challenge_required_on_new_device: Option<bool>,
    device_only_remembered_on_user_prompt: Option<bool>,
}
