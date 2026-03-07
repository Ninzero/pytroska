"""Pytroska - High-performance MKV file processing library."""

from pytroska.exceptions import ParseError as ParseError
from pytroska.exceptions import PytroskaError as PytroskaError
from pytroska.exceptions import CorruptedError as CorruptedError
from pytroska.exceptions import UnsupportedError as UnsupportedError
from pytroska._pytroska_core import SegmentInfo as SegmentInfo
from pytroska._pytroska_core import core_version as core_version
from pytroska._pytroska_core import parse_segment_info as parse_segment_info

# TODO(Phase 5): 添加 types.py (TrackType 枚举)

__version__: str = '0.1.0'

__all__ = [
    'CorruptedError',
    'ParseError',
    'PytroskaError',
    'SegmentInfo',
    'UnsupportedError',
    '__version__',
    'core_version',
    'parse_segment_info',
]
