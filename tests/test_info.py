"""Tests for Segment Info parsing (Phase 4)."""

from __future__ import annotations

from pathlib import Path

import pytest

from pytroska import parse_segment_info


def test_info_duration_test1(test1_mkv_path: Path) -> None:
    """test1.mkv 应有 duration。"""
    info = parse_segment_info(test1_mkv_path)
    assert info.duration_raw is not None
    assert info.duration_raw > 0


def test_info_timecode_scale_default(test1_mkv_path: Path) -> None:
    """test1.mkv 使用默认 TimestampScale = 1,000,000。"""
    info = parse_segment_info(test1_mkv_path)
    assert info.timecode_scale == 1_000_000


def test_info_timecode_scale_custom(test2_mkv_path: Path) -> None:
    """test2.mkv 使用自定义 TimestampScale = 100,000。"""
    info = parse_segment_info(test2_mkv_path)
    assert info.timecode_scale == 100_000


def test_info_muxing_app(test1_mkv_path: Path) -> None:
    """muxing_app 应为非空字符串。"""
    info = parse_segment_info(test1_mkv_path)
    assert isinstance(info.muxing_app, str)
    assert len(info.muxing_app) > 0


def test_info_writing_app(test1_mkv_path: Path) -> None:
    """writing_app 应为非空字符串。"""
    info = parse_segment_info(test1_mkv_path)
    assert isinstance(info.writing_app, str)
    assert len(info.writing_app) > 0


def test_info_date_utc_raw_type(test1_mkv_path: Path) -> None:
    """date_utc_raw 应为 int 或 None。"""
    info = parse_segment_info(test1_mkv_path)
    assert info.date_utc_raw is None or isinstance(info.date_utc_raw, int)


def test_info_segment_uid_hex(test1_mkv_path: Path) -> None:
    """segment_uid 如存在应为 32 字符的 hex 字符串。"""
    info = parse_segment_info(test1_mkv_path)
    if info.segment_uid is not None:
        assert len(info.segment_uid) == 32
        int(info.segment_uid, 16)  # 验证是合法 hex


def test_info_field_types(test1_mkv_path: Path) -> None:
    """验证所有字段的类型正确。"""
    info = parse_segment_info(test1_mkv_path)
    assert isinstance(info.timecode_scale, int)
    assert isinstance(info.muxing_app, str)
    assert isinstance(info.writing_app, str)
    assert info.title is None or isinstance(info.title, str)


def test_info_repr(test1_mkv_path: Path) -> None:
    """__repr__ 不崩溃。"""
    info = parse_segment_info(test1_mkv_path)
    r = repr(info)
    assert 'SegmentInfo' in r


def test_info_nonexistent_file() -> None:
    """不存在的文件应抛出 FileNotFoundError。"""
    with pytest.raises(FileNotFoundError, match='No such file'):
        parse_segment_info('/nonexistent/video.mkv')
