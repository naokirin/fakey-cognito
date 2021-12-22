use heck::AsSnakeCase;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::Path;

const DEFAULT_HOOK_DIR_PATH: &str = "hooks";

pub fn call_request_hook<T>(
    action: &str,
    value: &T,
    hooks_dir: Option<&str>,
) -> Result<String, PyErr>
where
    T: serde::Serialize,
{
    let pyname = AsSnakeCase(action);
    let dir = hooks_dir.unwrap_or(DEFAULT_HOOK_DIR_PATH);
    let path = format!("{}/{}.py", dir, pyname);
    if !Path::new(&path).exists() {
        return Ok("{}".to_string());
    }

    Python::with_gil(|py| {
        let syspath: &PyList = py
            .import("sys")
            .unwrap()
            .getattr("path")
            .unwrap()
            .try_into()
            .unwrap();
        // error[E0277]: the trait bound `std::path::Display<'_>: pyo3::conversion::ToPyObject` is not satisfied
        // syspath.insert(0, path.display()).unwrap();
        syspath.insert(0, dir).unwrap();

        let hook = py.import(format!("{}", AsSnakeCase(action)).as_str())?;
        let arg = serde_json::to_string(value).unwrap_or_else(|_| "".to_string());
        hook.getattr("hook")?.call1((arg,))?.extract()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn call_hook_test() {
        let result = call_request_hook("AdminGetUser", &"{}", Some("resources/test/hooks"));
        assert!(result.is_ok());
        assert_eq!("{ \"foo\": \"bar\" }", result.unwrap());
    }
}
