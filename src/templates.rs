use serde::Serialize;
use tera::{Context, Tera};
use tokio::sync::OnceCell;

const TEMPLATE_PATH: &'static str = "templates";
const DEFAULT_TEMPLATE_PATH: &'static str = "resources/default_templates";
static TEMPLATES: OnceCell<Option<Tera>> = OnceCell::const_new();
static DEFAULT_TEMPLATES: OnceCell<Option<Tera>> = OnceCell::const_new();

pub async fn init_template(dir_path: Option<&str>) {
    let dir_path = dir_path.unwrap_or(TEMPLATE_PATH);
    let path = format!("{}/**/*", dir_path);
    log::debug!("Load templates: {}", path);
    TEMPLATES
        .get_or_init(|| async move {
            let t = Tera::new(path.as_str());
            if let Err(e) = t.as_ref() {
                log::warn!("{}", e);
            }
            t.ok()
        })
        .await;
}

pub async fn init_default_template() {
    let dir_path = DEFAULT_TEMPLATE_PATH;
    let path = format!("{}/**/*", dir_path);
    log::debug!("Load default templates: {}", path);
    DEFAULT_TEMPLATES
        .get_or_init(|| async move {
            let t = Tera::new(path.as_str());
            if let Err(e) = t.as_ref() {
                log::warn!("{}", e);
            }
            t.ok()
        })
        .await;
}

pub fn render_template<T>(action_name: &str, context_value: &T) -> Option<String>
where
    T: Serialize,
{
    let file_name = format!("user_pools/{}.json", action_name);
    let r = serde_json::to_string(&context_value).unwrap();
    let map: std::collections::HashMap<String, serde_json::Value> =
        serde_json::from_str(&r).unwrap();
    if let Some(o) = TEMPLATES.get() {
        let t = render_template_internal(&o, file_name.as_str(), &map);
        if let Ok(_) = t {
            return t.ok();
        }
    }
    if let Some(o) = DEFAULT_TEMPLATES.get() {
        return render_template_internal(&o, file_name.as_str(), &map).ok();
    }
    log::error!("Template '{}' not found", file_name);
    None
}

fn render_template_internal(
    tera: &Option<Tera>,
    file_name: &str,
    context_values: &std::collections::HashMap<String, serde_json::Value>,
) -> std::result::Result<String, tera::Error> {
    let t = tera.as_ref();
    if let None = t {
        return Err(tera::Error::msg("No initialized tera templates"));
    }
    // NOTE: Manually inserts to a context because tera does not treat an option value.
    let mut context = Context::new();
    for (k, v) in context_values.iter() {
        context.insert(k, &v);
    }
    t.unwrap().render(file_name, &context)
}
