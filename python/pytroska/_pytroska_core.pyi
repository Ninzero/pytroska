"""Type stubs for pytroska._pytroska_core Rust extension module."""

from os import PathLike

def core_version() -> str: ...

class PytroskaError(Exception): ...
class ParseError(PytroskaError): ...
class CorruptedError(PytroskaError): ...
class UnsupportedError(PytroskaError): ...

class EbmlHeader:
    version: int
    read_version: int
    max_id_length: int
    max_size_length: int
    doc_type: str
    doc_type_version: int
    doc_type_read_version: int
    def __repr__(self) -> str: ...

def parse_ebml_header(path: str | PathLike[str]) -> EbmlHeader: ...

class SegmentInfo:
    duration_raw: float | None
    timecode_scale: int
    title: str | None
    muxing_app: str
    writing_app: str
    date_utc_raw: int | None
    segment_uid: str | None
    def __repr__(self) -> str: ...

def parse_segment_info(path: str | PathLike[str]) -> SegmentInfo: ...
