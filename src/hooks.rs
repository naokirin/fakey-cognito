use heck::SnakeCase;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::Path;

const DEFAULT_HOOK_DIR_PATH: &str = "hooks";

pub fn call_request_hook<T>(action: &str, value: &T) -> Result<String, PyErr>
where
    T: serde::Serialize,
{
    let pyname = action.to_snake_case();
    let dir = crate::opts::get_opt_hooks().map_or(DEFAULT_HOOK_DIR_PATH, |o| o);
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

        let hook = py.import(action.to_snake_case().as_str())?;
        let arg = serde_json::to_string(value).unwrap_or_else(|_| "".to_string());
        hook.getattr("hook")?.call1((arg,))?.extract()
    })
}
