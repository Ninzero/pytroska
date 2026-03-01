use pyo3::prelude::*;

/// Returns the version string of the pytroska Rust core library.
#[pyfunction]
fn core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// PyO3 module definition for pytroska's Rust core.
#[pymodule]
fn _pytroska_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    Ok(())
}
