use pyo3::PyErr;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use webm_iterable::errors::TagIteratorError;

// Python 异常类（via create_exception!，第一参数为 __module__ 属性）
create_exception!(pytroska._pytroska_core, PytroskaError, PyException);
create_exception!(pytroska._pytroska_core, ParseError, PytroskaError);
create_exception!(pytroska._pytroska_core, CorruptedError, PytroskaError);
create_exception!(pytroska._pytroska_core, UnsupportedError, PytroskaError);

/// Rust 内部错误枚举。
///
/// 命名为 `PytroskaRustError` 是因为 `PytroskaError` 已被
/// `create_exception!` 宏占用（作为 Python 异常类）。
#[derive(Debug, thiserror::Error)]
pub enum PytroskaRustError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at position {position}: {message}")]
    Parse { position: u64, message: String },

    #[error("Invalid EBML header: {0}")]
    InvalidHeader(String),

    #[error("Unsupported DocType: expected 'matroska' or 'webm', got '{0}'")]
    UnsupportedDocType(String),

    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    #[error("Corrupted data: {0}")]
    Corrupted(String),
}

impl From<PytroskaRustError> for PyErr {
    fn from(err: PytroskaRustError) -> PyErr {
        let message = err.to_string();
        match err {
            PytroskaRustError::Io(e) => PyErr::from(e),
            PytroskaRustError::Parse { .. } | PytroskaRustError::InvalidHeader(_) => {
                ParseError::new_err(message)
            }
            PytroskaRustError::UnsupportedDocType(_) | PytroskaRustError::Unsupported(_) => {
                UnsupportedError::new_err(message)
            }
            PytroskaRustError::Corrupted(_) => CorruptedError::new_err(message),
        }
    }
}

pub(crate) fn map_tag_iterator_error(e: TagIteratorError) -> PytroskaRustError {
    let message = e.to_string();
    match e {
        TagIteratorError::ReadError { source } => PytroskaRustError::Io(source),
        TagIteratorError::CorruptedFileData(_) | TagIteratorError::CorruptedTagData { .. } => {
            PytroskaRustError::Corrupted(message)
        }
        TagIteratorError::UnexpectedEOF { tag_start, .. } => PytroskaRustError::Parse {
            position: tag_start as u64,
            message,
        },
    }
}
