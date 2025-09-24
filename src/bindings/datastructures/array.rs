use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use numpy::{ IntoPyArray, PyArray1, PyReadonlyArray1, PyUntypedArrayMethods };

use crate::core::datastructures::array::{ ArrayLike, RollingMean, rolling_mean };

impl<'a> ArrayLike<f64> for PyReadonlyArray1<'a, f64> {
    fn len(&self) -> usize {
        self.shape()[0]
    }

    fn as_slice(&self) -> &[f64] {
        // SAFETY: unwrap is safe if array is contiguous (NumPy enforces this when read-only)
        self.as_slice().expect("NumPy array must be contiguous")
    }

    fn as_aligned_slice(&self) -> Option<&[f64]> {
        Some(self.as_slice().ok()?)
    }
}

/// A streaming rolling mean calculator with fixed window size.
///
/// Efficient for online/incremental data, supports push & reset.
#[pyclass(name = "RollingMean")]
struct RollingMeanWrapper {
    inner: RollingMean,
}

#[pymethods]
impl RollingMeanWrapper {
    /// Creates a new rolling mean filter with specified window size.
    #[new]
    fn new(window: usize) -> PyResult<Self> {
        if window == 0 {
            return Err(PyValueError::new_err("window must be > 0"));
        }
        Ok(Self {
            inner: RollingMean::new(window),
        })
    }

    /// Pushes a single value and returns the updated rolling mean.
    fn push(&mut self, value: f64) -> f64 {
        self.inner.push(value)
    }

    /// Pushes a batch of values from a NumPy array.
    fn push_batch(&mut self, data: PyReadonlyArray1<f64>) {
        self.inner.push_batch(&data);
    }

    /// Returns the current mean of the window.
    fn current(&self) -> f64 {
        self.inner.current_mean()
    }

    /// Clears internal buffer and resets the rolling mean.
    fn reset(&mut self) {
        self.inner.reset();
    }
}

/// Computes rolling mean over a NumPy array (float64).
///
/// Parameters:
/// - `data`: 1D NumPy array (float64)
/// - `window`: window size (must be > 0)
///
/// Returns:
/// - 1D NumPy array (same length) of rolling means
#[pyfunction(name = "rolling_mean")]
fn rolling_mean_func<'py>(
    py: Python<'py>,
    data: PyReadonlyArray1<'py, f64>,
    window: usize
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    if window == 0 {
        return Err(PyValueError::new_err("window must be > 0"));
    }

    let result = rolling_mean(&data, window);
    Ok(result.into_pyarray(py))
}

/// Register this module to Python.
pub fn register_arrays(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RollingMeanWrapper>()?;
    m.add_function(wrap_pyfunction!(rolling_mean_func, m)?)?;
    Ok(())
}
