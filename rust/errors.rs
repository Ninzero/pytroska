use pyo3::PyErr;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyIOError};

// Python 异常类（via create_exception!，第一参数为 __module__ 属性）
create_exception!(_pytroska_core, PytroskaError, PyException);
create_exception!(_pytroska_core, ParseError, PytroskaError);
create_exception!(_pytroska_core, CorruptedError, PytroskaError);
create_exception!(_pytroska_core, UnsupportedError, PytroskaError);

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
        match err {
            PytroskaRustError::Io(e) => PyIOError::new_err(e.to_string()),
            PytroskaRustError::Parse { position, message } => {
                ParseError::new_err(format!("Parse error at position {position}: {message}"))
            }
            PytroskaRustError::InvalidHeader(msg) => ParseError::new_err(msg),
            PytroskaRustError::UnsupportedDocType(dt) => UnsupportedError::new_err(format!(
                "Unsupported DocType: expected 'matroska' or 'webm', got '{dt}'"
            )),
            PytroskaRustError::Unsupported(msg) => UnsupportedError::new_err(msg),
            PytroskaRustError::Corrupted(msg) => CorruptedError::new_err(msg),
        }
    }
}
