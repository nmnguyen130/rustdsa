use pyo3::prelude::*;

pub mod sorting;

/// Registers all algorithm-related submodules under the given Python module.
pub fn register_algorithms(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let sorting_mod = PyModule::new(m.py(), "sorting")?;
    sorting::register_sorts(&sorting_mod)?;
    m.add_submodule(&sorting_mod)?;
    Ok(())
}
