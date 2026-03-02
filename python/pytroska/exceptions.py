"""Pytroska exception hierarchy, re-exported from the Rust core."""

from pytroska._pytroska_core import ParseError as ParseError
from pytroska._pytroska_core import PytroskaError as PytroskaError
from pytroska._pytroska_core import CorruptedError as CorruptedError
from pytroska._pytroska_core import UnsupportedError as UnsupportedError

__all__ = ['CorruptedError', 'ParseError', 'PytroskaError', 'UnsupportedError']
