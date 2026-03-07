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
        // TODO(Phase 5): 勿直接追加 Tracks(Master::Start) 后顺序调用 extract_tracks。
        //   RFC 9559 不保证 Info 先于 Tracks；若文件中 Tracks 先出现，extract_info 的
        //   `_ => {}` 会原子消费 Tracks(Full(...))，extract_tracks 随后将找不到 Tracks。
        //   正确方案：将 open() 改为单次循环同时收集 Info 和 Tracks（顺序无关）：
        //     loop { match tag {
        //       Info(Full(ch))   => info_opt   = Some(parse_info_children(&ch)),
        //       Tracks(Full(ch)) => tracks_opt = Some(parse_tracks_children(&ch)),
        //       Cluster(_) | None => break,
        //       _ => {}
        //     }}
        //   parse_info_children / parse_tracks_children 作为纯函数分别处理子元素列表。
        let mut iter = WebmIterator::new(&mut buf_reader, &[MatroskaSpec::Info(Master::Start)]);

        let header = parse_header_from_iter(&mut iter, &path_display)?;
        let info = extract_info(&mut iter, &path_display)?;

        Ok(Self { header, info })
    }
}
