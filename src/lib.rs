use pyo3::prelude::*;

mod algorithms;
use algorithms::sorting;

/// Quicksort Algorithm
#[pyfunction]
fn quicksort(values: Vec<i64>) -> Vec<i64> {
    sorting::quicksort(values)
}

/// Baseline using Rust's std sort for benchmarking
#[pyfunction]
fn quicksort_std(values: Vec<i64>) -> Vec<i64> {
    sorting::quicksort_std(values)
}

/// Python module declaration
#[pymodule]
fn rustdsa(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Create submodule "sorting"
    let sorting_mod = PyModule::new(m.py(), "sorting")?;

    sorting_mod.add_function(wrap_pyfunction!(quicksort, &sorting_mod)?)?;
    sorting_mod.add_function(wrap_pyfunction!(quicksort_std, &sorting_mod)?)?;

    // Add the submodule to the main module
    m.add("sorting", sorting_mod)?;

    Ok(())
}
