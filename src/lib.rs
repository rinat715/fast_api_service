use pyo3::prelude::*;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::hash::{Hash, Hasher};

#[derive(Deserialize)]
#[serde(transparent)]
struct RawValueHashed<'a> {
    #[serde(borrow)]
    raw_value: Vec<&'a RawValue>,
}

impl<'a> Hash for RawValueHashed<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw_value.get().hash(state);
    }
}

fn json_loads(raw: &str) -> serde_json::Result<RawValueHashed> {
    let _req: RawValueHashed = serde_json::from_str(raw)?;

    Ok(_req)
}

fn permutate(items: &Vec<&RawValueHashed>) {
    for perm in items.iter().permutations(items.len()).unique() {
        println!("{:?}", perm);
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn service_permutations_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
