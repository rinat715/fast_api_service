use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::Python;

use core::result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
struct RawValueHashed<'a> {
    #[serde(borrow)]
    raw_value: &'a RawValue,
}

impl<'a> Hash for RawValueHashed<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw_value.get().hash(state);
    }
}

impl<'a> PartialEq for RawValueHashed<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value.get() == other.raw_value.get()
    }
}

impl<'a> Eq for RawValueHashed<'a> {}

struct WrapJsonError(serde_json::Error);

impl From<WrapJsonError> for PyErr {
    fn from(error: WrapJsonError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<serde_json::Error> for WrapJsonError {
    fn from(other: serde_json::Error) -> Self {
        Self(other)
    }
}

type Result<T> = result::Result<T, WrapJsonError>;

fn permutations(arg: &str) -> Result<String> {
    let json: Vec<RawValueHashed> = serde_json::from_str(arg)?;

    let items = json
        .iter()
        .permutations(json.len())
        .unique()
        .collect::<Vec<Vec<&RawValueHashed>>>();

    Ok(serde_json::to_string(&items)?)
}

#[pyfunction]
fn get(raw: &str) -> PyResult<String> {
    Ok(permutations(raw)?)
}

/// A Python module implemented in Rust.
#[pymodule]
fn service_permutations_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let arg = String::from(r#"[1,2,3]"#);
        let result = permutations(&arg).unwrap_or_default();

        assert_eq!(
            result,
            r#"[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]"#
        );
    }

    #[test]
    fn test_json_err() {
        let arg = String::from(r#"err"#);
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let err = get(&arg).unwrap_err();
            assert!(err.is_instance_of::<PyValueError>(py));
            assert_eq!(
                err.value(py).to_string(),
                "expected value at line 1 column 1"
            );
        });
    }
}
