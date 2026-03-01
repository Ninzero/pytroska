# Test Suite — CLAUDE.md

[Root](../../CLAUDE.md) > **tests/**

---

## Module Responsibility

`tests/` contains the pytest test suite for Pytroska. Tests are written in Python and exercise both the Rust extension module and the Python wrapper layer. The suite is designed to grow incrementally across implementation phases.

---

## Current State (Phase 1)

Only `test_smoke.py` exists. It verifies the most basic invariants: the package can be imported, `core_version()` returns a well-formed semver string, and `__version__` matches `"0.1.0"`.

```python
# tests/test_smoke.py
def test_import() -> None: ...
def test_core_version_returns_string() -> None: ...
def test_core_version_format() -> None: ...
def test_package_version() -> None: ...
```

---

## Test Infrastructure (Phase 3)

`tests/conftest.py` will auto-download the 8 official Matroska test files from `https://github.com/ietf-wg-cellar/matroska-test-files` into `tests/fixtures/` on first run. Only `urllib.request` is used (no extra dependency).

Fixtures provided: `test1_mkv_path`, `test2_mkv_path`, ..., `test8_mkv_path` — each returns a `pathlib.Path`.

Known characteristics of the test corpus:

| File | Notable property |
|------|-----------------|
| test1.mkv | 2 tracks (1 video + 1 audio), standard timecode scale 1,000,000 |
| test2.mkv | Custom timecode scale 100,000 |
| test4.mkv | Live stream (may have no Duration field) |
| test5.mkv | 10 tracks (1 video + 2 audio + 7 subtitles), subtitle languages |

---

## Planned Test Files by Phase

| File | Phase | What it tests |
|------|-------|--------------|
| `test_smoke.py` | 1 | Basic import and `core_version()` |
| `test_errors.py` | 2 | Exception hierarchy and `isinstance` checks |
| `test_header.py` | 3 | EBML header field values, invalid file errors |
| `test_info.py` | 4 | `SegmentInfo` duration, timecode scale, muxing/writing app |
| `test_tracks.py` | 5 | Track counts, codec IDs, video resolution, audio channels, track flags |
| `test_file.py` | 6 | `MKVFile` API: open, context manager, `duration`, `title`, filtering |
| `test_demux.py` | 8 | Frame iteration: `BlockData` types and timestamps |
| `test_cues.py` | 9 | Cue index lookup correctness |
| `test_seek.py` | 9 | Seek-to-timestamp accuracy |
| `test_chapters.py` | 10 | Chapter title and timestamp extraction |
| `test_tags.py` | 10 | Tag key/value extraction |
| `test_attachments.py` | 10 | Attachment name, MIME type, and raw bytes |
| `test_integration.py` | 11 | Full pipeline over all 8 test files |

---

## Running Tests

```bash
# All tests:
uv run pytest tests/ -v

# Single phase:
uv run pytest tests/test_smoke.py -v

# With coverage (if pytest-cov added later):
uv run pytest tests/ --cov=pytroska --cov-report=term-missing
```

pytest is configured in `pyproject.toml`:

```toml
[tool.pytest.ini_options]
testpaths = ["tests"]
```

---

## Related File List

- `/Users/Ninzero/Documents/python_program/pytroska/tests/test_smoke.py` — Phase 1 smoke tests (exists)
- `/Users/Ninzero/Documents/python_program/pytroska/tests/conftest.py` — fixtures and test file downloads (Phase 3, planned)
- `/Users/Ninzero/Documents/python_program/pytroska/tests/fixtures/` — downloaded Matroska test MKV files (Phase 3, planned)
- `/Users/Ninzero/Documents/python_program/pytroska/pyproject.toml` — pytest configuration

---

## Changelog

| Date | Description |
|------|-------------|
| 2026-03-02 | Module CLAUDE.md created. Phase 1 state: `test_smoke.py` present. All other test files planned per phase. |
