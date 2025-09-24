use pyo3::prelude::*;

pub mod array;

pub fn register_datastructures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    array::register_arrays(m)?;

    Ok(())
}
