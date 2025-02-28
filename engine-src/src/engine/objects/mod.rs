use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod tengine_place_img;

/// Declaration of the objects submodule for the parent python module.
/// Reference: https://pyo3.rs/v0.23.4/module.html
pub fn register_objects_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "objects")?;
    child_module.add_function(wrap_pyfunction!(tengine_place_img::tengine_place_img, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}