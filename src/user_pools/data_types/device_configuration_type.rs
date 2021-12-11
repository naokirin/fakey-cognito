use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceConfigurationType {
    challenge_required_on_new_device: Option<bool>,
    device_only_remembered_on_user_prompt: Option<bool>,
}
