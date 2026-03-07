use pyo3::prelude::*;
use webm_iterable::WebmIterator;
use webm_iterable::matroska_spec::{Master, MatroskaSpec};

use crate::errors::{PytroskaRustError, map_tag_iterator_error};

#[pyclass(frozen, get_all, skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct SegmentInfo {
    pub duration_raw: Option<f64>,
    pub timecode_scale: u64,
    pub title: Option<String>,
    pub muxing_app: String,
    pub writing_app: String,
    pub date_utc_raw: Option<i64>,
    pub segment_uid: Option<String>,
}

#[pymethods]
impl SegmentInfo {
    fn __repr__(&self) -> String {
        format!(
            "SegmentInfo(duration_raw={:?}, timecode_scale={}, title={:?})",
            self.duration_raw, self.timecode_scale, self.title
        )
    }
}

fn parse_info_children(children: &[MatroskaSpec]) -> SegmentInfo {
    let mut duration_raw: Option<f64> = None;
    let mut timecode_scale: u64 = 1_000_000;
    let mut title: Option<String> = None;
    let mut muxing_app = String::new();
    let mut writing_app = String::new();
    let mut date_utc_raw: Option<i64> = None;
    let mut segment_uid: Option<String> = None;

    for child in children {
        match child {
            MatroskaSpec::TimestampScale(v) => timecode_scale = *v,
            MatroskaSpec::Duration(v) => duration_raw = Some(*v),
            MatroskaSpec::Title(s) => title = Some(s.clone()),
            MatroskaSpec::MuxingApp(s) => muxing_app = s.clone(),
            MatroskaSpec::WritingApp(s) => writing_app = s.clone(),
            MatroskaSpec::DateUTC(v) => date_utc_raw = Some(*v),
            MatroskaSpec::SegmentUID(v) => {
                if v.len() == 16 {
                    segment_uid = Some(v.iter().map(|b| format!("{b:02x}")).collect());
                } else {
                    tracing::warn!(
                        len = v.len(),
                        "SegmentUID has invalid length (expected 16 bytes), ignoring"
                    );
                }
            }
            _ => {}
        }
    }

    SegmentInfo {
        duration_raw,
        timecode_scale,
        title,
        muxing_app,
        writing_app,
        date_utc_raw,
        segment_uid,
    }
}

/// 从已配置了 `Info` 缓冲的 WebmIterator 中提取 Segment Info 元素。
///
/// **前置条件**：调用方创建 iterator 时必须将 `Info` 加入 `tags_to_buffer`，
/// 使 Info 以 `Master::Full(children)` 一次性返回。
///
/// # Phase 5 注意
///
/// TODO(Phase 5): 此函数在 Phase 4 由 `MkvReader::open()` 直接调用。
/// Phase 5 **不能**保留 `extract_info → extract_tracks` 顺序调用模式——
/// RFC 9559 不保证 Info 先于 Tracks，若 Tracks 先出现则会被此函数的
/// `_ => {}` 分支原子消费，`extract_tracks` 随后将找不到 Tracks。
/// Phase 5 应在 `open()` 内改用单次循环（见 `reader.rs` TODO）。
/// 此函数可保留，用于不需要 Tracks 的独立调用场景（如 `parse_segment_info`）。
pub(crate) fn extract_info<R: std::io::Read>(
    iter: &mut WebmIterator<R>,
    path_display: &str,
) -> Result<SegmentInfo, PytroskaRustError> {
    loop {
        let Some(tag_result) = iter.next() else {
            return Err(PytroskaRustError::Parse {
                position: iter.last_emitted_tag_offset() as u64,
                message: format!("Missing required Info element in '{path_display}'"),
            });
        };

        let tag = tag_result.map_err(map_tag_iterator_error)?;
        match tag {
            MatroskaSpec::Info(Master::Full(children)) => {
                return Ok(parse_info_children(&children));
            }
            MatroskaSpec::Info(_) => {
                // Info was not buffered as Master::Full — iterator was not configured correctly
                return Err(PytroskaRustError::Unsupported(
                    "Internal parser invariant violated: Info must be buffered as Master::Full"
                        .to_string(),
                ));
            }
            MatroskaSpec::Cluster(_) => {
                return Err(PytroskaRustError::Parse {
                    position: iter.last_emitted_tag_offset() as u64,
                    message: format!("Missing required Info element in '{path_display}'"),
                });
            }
            _ => {}
        }
    }
}
