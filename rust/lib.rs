mod errors;
mod header;
mod info;
mod reader;

use std::path::PathBuf;

use errors::{CorruptedError, ParseError, PytroskaError, UnsupportedError};
use header::{EbmlHeader, parse_ebml_header};
use info::SegmentInfo;
use pyo3::prelude::*;
use reader::MkvReader;

/// Returns the version string of the pytroska Rust core library.
#[pyfunction]
fn core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// 解析 MKV 文件的 Segment Info。
#[pyfunction]
fn parse_segment_info(path: PathBuf) -> PyResult<SegmentInfo> {
    let reader = MkvReader::open(&path)?;
    Ok(reader.info)
}

/// PyO3 module definition for pytroska's Rust core.
#[pymodule]
fn _pytroska_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    // 注册 Python 异常类
    m.add("PytroskaError", py.get_type::<PytroskaError>())?;
    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("CorruptedError", py.get_type::<CorruptedError>())?;
    m.add("UnsupportedError", py.get_type::<UnsupportedError>())?;
    // Phase 1
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    // Phase 3
    m.add_class::<EbmlHeader>()?;
    m.add_function(wrap_pyfunction!(parse_ebml_header, m)?)?;
    // Phase 4
    m.add_class::<SegmentInfo>()?;
    m.add_function(wrap_pyfunction!(parse_segment_info, m)?)?;
    Ok(())
}
