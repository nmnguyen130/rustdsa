use pyo3::prelude::*;

mod core;
mod bindings;

/// Python module declaration
#[pymodule]
fn rustdsa(m: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register_bindings(m)?;
    Ok(())
}
