use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::exceptions;
use pyo3::prelude::*;

pub mod rust_fn;

impl From<rust_fn::AnnotatorError> for PyErr {
    fn from(err: rust_fn::AnnotatorError) -> Self {
        match err {
            rust_fn::AnnotatorError::ShapeError(e) => {
                PyErr::new::<exceptions::PyValueError, _>(format!("{}", e))
            }
            rust_fn::AnnotatorError::MiscError(e) => {
                PyErr::new::<exceptions::PyValueError, _>(e)
            }
        }
    }
}

/// Annotate in-memory scanline data.
///
/// Args:
///     data (ndarray): 2D numpy array of scanline data (shape N x M).
///
/// Returns:
///     ndarray: Processed and annotated scanline data array.
#[pyfunction]
fn annotate_scanlines<'py>(
    py: Python<'py>,
    data: PyReadonlyArray2<'py, f64>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let array_view = data.as_array();
    let rs_result = rust_fn::annotate_scanlines(array_view)?;
    let py_result = rs_result.to_pyarray(py);
    Ok(py_result)
}

/// A library for processing and annotating raster scanlines in powder bed fusion data.
#[pymodule]
fn scanline_annotator(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(annotate_scanlines, m)?)?;
    Ok(())
}
