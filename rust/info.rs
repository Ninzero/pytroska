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
                segment_uid = Some(v.iter().map(|b| format!("{b:02x}")).collect());
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
