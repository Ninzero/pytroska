"""Pytroska - High-performance MKV file processing library."""

from pytroska.exceptions import ParseError as ParseError
from pytroska.exceptions import PytroskaError as PytroskaError
from pytroska.exceptions import CorruptedError as CorruptedError
from pytroska.exceptions import UnsupportedError as UnsupportedError
from pytroska._pytroska_core import core_version as core_version

__version__: str = '0.1.0'

__all__ = [
    'CorruptedError',
    'ParseError',
    'PytroskaError',
    'UnsupportedError',
    '__version__',
    'core_version',
]
