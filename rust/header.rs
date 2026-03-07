use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use pyo3::prelude::*;
use webm_iterable::WebmIterator;
use webm_iterable::matroska_spec::MatroskaSpec;

use crate::errors::{PytroskaRustError, map_tag_iterator_error};

// skip_from_py_object: EbmlHeader is constructed only by Rust; no Python-side constructor needed
#[pyclass(frozen, get_all, skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct EbmlHeader {
    pub version: u64,
    pub read_version: u64,
    pub max_id_length: u64,
    pub max_size_length: u64,
    pub doc_type: String,
    pub doc_type_version: u64,
    pub doc_type_read_version: u64,
}

#[pymethods]
impl EbmlHeader {
    fn __repr__(&self) -> String {
        format!(
            "EbmlHeader(doc_type={:?}, version={}, doc_type_version={})",
            self.doc_type, self.version, self.doc_type_version
        )
    }
}

pub(crate) fn parse_header_from_iter<R: std::io::Read>(
    iter: &mut WebmIterator<R>,
    path_display: &str,
) -> Result<EbmlHeader, PytroskaRustError> {
    // EBML spec defaults
    let mut version: u64 = 1;
    let mut read_version: u64 = 1;
    let mut max_id_length: u64 = 4;
    let mut max_size_length: u64 = 8;
    let mut doc_type: Option<String> = None;
    let mut doc_type_version: u64 = 1;
    let mut doc_type_read_version: u64 = 1;

    for tag in iter {
        let tag = tag.map_err(map_tag_iterator_error)?;

        match tag {
            MatroskaSpec::Segment(_) => break,
            MatroskaSpec::EbmlVersion(v) => version = v,
            MatroskaSpec::EbmlReadVersion(v) => read_version = v,
            MatroskaSpec::EbmlMaxIdLength(v) => max_id_length = v,
            MatroskaSpec::EbmlMaxSizeLength(v) => max_size_length = v,
            MatroskaSpec::DocType(s) => doc_type = Some(s.to_string()),
            MatroskaSpec::DocTypeVersion(v) => doc_type_version = v,
            MatroskaSpec::DocTypeReadVersion(v) => doc_type_read_version = v,
            _ => {}
        }
    }

    let doc_type = doc_type.ok_or_else(|| {
        PytroskaRustError::InvalidHeader(format!(
            "Missing required DocType element in '{path_display}'"
        ))
    })?;

    if doc_type != "matroska" && doc_type != "webm" {
        return Err(PytroskaRustError::UnsupportedDocType(doc_type));
    }

    Ok(EbmlHeader {
        version,
        read_version,
        max_id_length,
        max_size_length,
        doc_type,
        doc_type_version,
        doc_type_read_version,
    })
}

#[pyfunction]
pub fn parse_ebml_header(path: PathBuf) -> PyResult<EbmlHeader> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut iter = WebmIterator::new(&mut reader, &[]);
    let header = parse_header_from_iter(&mut iter, &path.display().to_string())?;
    Ok(header)
}
