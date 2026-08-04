use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};
use pyo3::exceptions;
use pyo3::prelude::*;

pub mod rust_fn;

impl From<rust_fn::AnnotatorError> for PyErr {
    fn from(err: rust_fn::AnnotatorError) -> Self {
        match err {
            rust_fn::AnnotatorError::ShapeError(e) => {
                PyErr::new::<exceptions::PyValueError, _>(format!("{}", e))
            }
            rust_fn::AnnotatorError::MiscError(e) => PyErr::new::<exceptions::PyValueError, _>(e),
        }
    }
}

/// Annotate in-memory scanline data from 1D x and y coordinate arrays.
///
/// Args:
///     x (ndarray): 1D numpy array of x-coordinates (float32 or float64).
///     y (ndarray): 1D numpy array of y-coordinates (float32 or float64).
///
/// Returns:
///     ndarray: 1D numpy array of scanline_id assignments (int32).
#[pyfunction]
fn annotate_scanlines<'py>(
    py: Python<'py>,
    x: &Bound<'py, PyAny>,
    y: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyArray1<i32>>> {
    if let (Ok(x_f64), Ok(y_f64)) = (
        x.extract::<PyReadonlyArray1<'py, f64>>(),
        y.extract::<PyReadonlyArray1<'py, f64>>(),
    ) {
        let rs_result = rust_fn::annotate_scanlines(x_f64.as_array(), y_f64.as_array())?;
        Ok(rs_result.to_pyarray(py))
    } else if let (Ok(x_f32), Ok(y_f32)) = (
        x.extract::<PyReadonlyArray1<'py, f32>>(),
        y.extract::<PyReadonlyArray1<'py, f32>>(),
    ) {
        let rs_result = rust_fn::annotate_scanlines(x_f32.as_array(), y_f32.as_array())?;
        Ok(rs_result.to_pyarray(py))
    } else {
        Err(PyErr::new::<exceptions::PyTypeError, _>(
            "x and y must both be 1D numpy arrays of float64 or float32 type",
        ))
    }
}

/// A library for processing and annotating raster scanlines in powder bed fusion data.
#[pymodule]
fn scanline_annotator(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(annotate_scanlines, m)?)?;
    Ok(())
}
