use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use webm_iterable::WebmIterator;
use webm_iterable::matroska_spec::{Master, MatroskaSpec};

use crate::errors::PytroskaRustError;
use crate::header::{EbmlHeader, parse_header_from_iter};
use crate::info::{SegmentInfo, extract_info};

pub(crate) struct MkvReader {
    #[allow(dead_code)] // Phase 5: MKVFile will use header
    pub header: EbmlHeader,
    pub info: SegmentInfo,
}

impl MkvReader {
    pub fn open(path: &Path) -> Result<Self, PytroskaRustError> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let path_display = path.display().to_string();

        // tags_to_buffer: 缓冲 Info 为 Master::Full，extract_info 可一次性读取所有子元素
        // Phase 5 扩展: 加入 MatroskaSpec::Tracks(Master::Start)
        let mut iter = WebmIterator::new(&mut buf_reader, &[MatroskaSpec::Info(Master::Start)]);

        let header = parse_header_from_iter(&mut iter, &path_display)?;
        let info = extract_info(&mut iter, &path_display)?;

        Ok(Self { header, info })
    }
}
