use fakey_cognito::*;

mod admin_add_user_to_group_test;
mod admin_confirm_sign_up_test;
mod admin_create_user_test;
mod admin_delete_user_attributes_test;
mod admin_delete_user_test;
mod admin_disable_provider_for_user_test;
mod admin_disable_user_test;
mod admin_enable_user_test;
mod admin_forget_device_test;
mod admin_get_device_test;

pub async fn setup() {
    opts::init_opt().await;
    let templates_opt = opts::get_opt_templates();
    tokio::join!(
        user_pools::init_config(opts::get_opt_config()),
        templates::init_template(templates_opt.map(String::as_str)),
        templates::init_default_template()
    );
}
