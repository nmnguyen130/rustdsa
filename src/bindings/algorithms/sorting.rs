use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::core::algorithms::sorting;

#[pyfunction(name = "introsort")]
fn py_introsort(mut values: Vec<i64>) -> PyResult<Vec<i64>> {
    sorting::introsort(&mut values);
    Ok(values)
}

#[pyfunction(name = "sort_std")]
fn py_sort_std(mut values: Vec<i64>) -> PyResult<Vec<i64>> {
    sorting::sort_std(&mut values);
    Ok(values)
}

pub fn register_sorts(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_introsort, m)?)?;
    m.add_function(wrap_pyfunction!(py_sort_std, m)?)?;
    Ok(())
}
