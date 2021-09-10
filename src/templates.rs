use serde::Serialize;
use tera::{Context, Tera};
use tokio::sync::OnceCell;

const TEMPLATE_PATH: &'static str = "./templates";
static TEMPLATES: OnceCell<Option<Tera>> = OnceCell::const_new();

pub async fn init_template(dir_path: Option<&str>) {
    let dir_path = dir_path.unwrap_or(TEMPLATE_PATH);
    let path = format!("{}/**/*.json", dir_path);
    TEMPLATES
        .get_or_init(|| async { Tera::new(path.as_str()).ok() })
        .await;
}

pub fn render_template<T>(file_name: &str, context_value: &T) -> Option<String>
where
    T: Serialize,
{
    if let Some(o) = TEMPLATES.get() {
        if let Some(t) = o {
            let context_opt = Context::from_serialize(&context_value).ok();
            if let Some(c) = context_opt {
                return t.render(file_name, &c).ok().map(escape_json_string);
            }
        }
    };
    None
}

fn escape_json_string(s: String) -> String {
    s.chars()
        .map(|c| {
            let s = c.to_string();
            let cs = match c {
                '"' => "\\\"",
                '\\' => "\\\\",
                '/' => "\\/",
                '\n' => "\\n",
                '\r' => "\\r",
                '\t' => "\\t",
                _ => s.as_str(),
            };
            cs.to_string()
        })
        .collect()
}
