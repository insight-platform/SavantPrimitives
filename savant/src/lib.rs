/// # C API for Savant Rust Library
///
pub mod capi;
/// # Basic objects
///
pub mod primitives;
pub mod test;
/// # Utility functions
///
pub mod utils;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

use primitives::message::video::query::py::video_object_query;

/// Returns the version of the package set in Cargo.toml
///
#[pyfunction]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[pymodule]
fn savant_rs(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(version, m)?)?;

    m.add_wrapped(wrap_pymodule!(primitives::primitives))?;
    m.add_wrapped(wrap_pymodule!(utils::utils))?;
    m.add_wrapped(wrap_pymodule!(video_object_query))?;

    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;

    sys_modules.set_item("savant_rs.primitives", m.getattr("primitives")?)?;
    sys_modules.set_item("savant_rs.utils", m.getattr("utils")?)?;
    sys_modules.set_item(
        "savant_rs.video_object_query",
        m.getattr("video_object_query")?,
    )?;

    Ok(())
}
