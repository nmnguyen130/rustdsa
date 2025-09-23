use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::algorithms::sorting;

#[pyfunction(name = "sort")]
fn py_sort(mut values: Vec<i64>) -> PyResult<Vec<i64>> {
    sorting::introsort(&mut values);
    Ok(values)
}

#[pyfunction(name = "sort_std")]
fn py_sort_std(mut values: Vec<i64>) -> PyResult<Vec<i64>> {
    sorting::sort_std(&mut values);
    Ok(values)
}

pub fn register_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_sort, m)?)?;
    m.add_function(wrap_pyfunction!(py_sort_std, m)?)?;
    Ok(())
}
