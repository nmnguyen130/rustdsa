use pyo3::prelude::*;

mod algorithms;
mod bindings;

/// Python module declaration
#[pymodule]
fn rustdsa(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Create submodule "sorting"
    let sorting_mod = PyModule::new(m.py(), "sorting")?;

    // Register algorithms
    bindings::sorting::register_py(&sorting_mod)?;

    // Add the submodule to the main module
    m.add_submodule(&sorting_mod)?;

    Ok(())
}
