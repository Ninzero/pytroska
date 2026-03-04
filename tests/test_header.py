"""Tests for EBML header parsing (Phase 3)."""

from pathlib import Path

import pytest

from pytroska.exceptions import ParseError, CorruptedError, UnsupportedError
from pytroska._pytroska_core import EbmlHeader, parse_ebml_header


def test_parse_header_test1(test1_mkv_path: Path) -> None:
    h = parse_ebml_header(test1_mkv_path)
    assert h.doc_type == 'matroska'
    assert h.version == 1
    assert h.max_id_length == 4
    assert h.max_size_length == 8


def test_parse_header_test5(test5_mkv_path: Path) -> None:
    h = parse_ebml_header(test5_mkv_path)
    assert h.doc_type == 'matroska'
    assert h.version == 1


def test_parse_header_accepts_str_and_pathlib(test1_mkv_path: Path) -> None:
    """parse_ebml_header should accept both str and pathlib.Path."""
    h_path = parse_ebml_header(test1_mkv_path)
    h_str = parse_ebml_header(str(test1_mkv_path))
    assert isinstance(h_path, EbmlHeader)
    assert isinstance(h_str, EbmlHeader)
    assert h_path.doc_type == h_str.doc_type


def test_parse_invalid_file(tmp_path: Path) -> None:
    # Random bytes are seen as corrupted EBML data, not a parse error
    fake = tmp_path / 'fake.mkv'
    fake.write_bytes(b'This is not an MKV file, just random bytes 0x00 0xff')
    with pytest.raises(CorruptedError):
        parse_ebml_header(fake)


def test_parse_nonexistent_file(tmp_path: Path) -> None:
    with pytest.raises(FileNotFoundError):
        parse_ebml_header(tmp_path / 'nonexistent.mkv')


def test_header_fields_types(test1_mkv_path: Path) -> None:
    h = parse_ebml_header(test1_mkv_path)
    assert isinstance(h.version, int)
    assert isinstance(h.read_version, int)
    assert isinstance(h.max_id_length, int)
    assert isinstance(h.max_size_length, int)
    assert isinstance(h.doc_type, str)
    assert isinstance(h.doc_type_version, int)
    assert isinstance(h.doc_type_read_version, int)


def test_header_repr(test1_mkv_path: Path) -> None:
    h = parse_ebml_header(test1_mkv_path)
    r = repr(h)
    assert 'EbmlHeader' in r
    assert 'matroska' in r


def test_parse_unsupported_doctype(tmp_path: Path) -> None:
    """DocType='avi' (not matroska/webm) should raise UnsupportedError.

    Minimal EBML bytes:
      1A 45 DF A3  EBML element ID
      86           VINT size=6
      42 82        DocType element ID
      83           VINT size=3
      61 76 69     "avi"
    """
    fake = tmp_path / 'bad_doctype.mkv'
    fake.write_bytes(b'\x1a\x45\xdf\xa3\x86\x42\x82\x83avi')
    with pytest.raises(UnsupportedError):
        parse_ebml_header(fake)


def test_parse_missing_doctype(tmp_path: Path) -> None:
    """EBML header without DocType element should raise ParseError.

    Minimal EBML bytes:
      1A 45 DF A3  EBML element ID
      80           VINT size=0 (empty EBML master)
      18 53 80 67  Segment element ID (triggers break)
      80           VINT size=0
    """
    fake = tmp_path / 'no_doctype.mkv'
    fake.write_bytes(b'\x1a\x45\xdf\xa3\x80\x18\x53\x80\x67\x80')
    with pytest.raises(ParseError):
        parse_ebml_header(fake)
