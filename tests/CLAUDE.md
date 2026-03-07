# Test Suite — CLAUDE.md

[Root](../CLAUDE.md) > **tests/**

---

## Module Responsibility

`tests/` contains the pytest test suite for Pytroska. Tests are written in Python and exercise both the Rust extension module and the Python wrapper layer. The suite is designed to grow incrementally across implementation phases.

---

## Current State (Phase 4 complete)

Five test files exist. The fixture infrastructure in `conftest.py` is fully operational.

| File | Phase | Status |
| ---- | ----- | ------ |
| `conftest.py` | 3 | Complete — auto-download + SHA-256 verify; fixtures test1..test8_mkv_path |
| `test_smoke.py` | 1 | Complete — import, `core_version()`, `__version__` |
| `test_errors.py` | 2 | Complete — exception hierarchy, inheritance, pickle round-trip |
| `test_header.py` | 3 | Complete — EBML header fields, error paths, unsupported DocType |
| `test_info.py` | 4 | Complete — SegmentInfo fields, timecode scale, SegmentUID hex |

---

## Test Infrastructure

### `conftest.py`

Session-scoped fixtures `test1_mkv_path` through `test8_mkv_path` (all returning `pathlib.Path`).

Download strategy:

1. Files downloaded to `tests/fixtures/` (or `$PYTROSKA_FIXTURES_DIR` env override).
2. SHA-256 hashes cached in `tests/fixtures/.hashes/testN.sha256`.
3. On each run, the existing file hash is verified against the cached hash; mismatches trigger a re-download.
4. Hash mismatch after re-download causes `pytest.fail()` with a clear message directing the user to delete the `.sha256` file to accept an updated version.

Uses only `urllib.request` — no extra dependency.

Known characteristics of the test corpus:

| File | Notable property |
| ---- | ---------------- |
| test1.mkv | 2 tracks (1 video + 1 audio), default `timecode_scale` 1,000,000 |
| test2.mkv | Custom `timecode_scale` = 100,000 |
| test4.mkv | Live stream — may have no `Duration` field |
| test5.mkv | 10 tracks (1 video + 2 audio + 7 subtitles), multiple languages |

### Test Coverage Per Phase

| File | Phase | What it tests |
| ---- | ----- | ------------- |
| `test_smoke.py` | 1 | `pytroska.__version__`, `core_version()` format (3-part semver) |
| `test_errors.py` | 2 | `issubclass` hierarchy, `__module__ == 'pytroska._pytroska_core'`, pickle round-trip |
| `test_header.py` | 3 | Field values on test1/test5, `str` and `Path` acceptance, `CorruptedError` on random bytes, `FileNotFoundError` on missing file, `UnsupportedError` on crafted EBML with `doc_type='avi'`, `__repr__` content |
| `test_info.py` | 4 | `duration_raw > 0` on test1, `timecode_scale == 1_000_000` on test1, `timecode_scale == 100_000` on test2, `muxing_app`/`writing_app` non-empty strings, `date_utc_raw` type, `segment_uid` is 32-char hex, `FileNotFoundError` on missing file |
| `test_tracks.py` | 5 | Track counts, codec IDs, video resolution, audio channels, track flags (planned) |
| `test_file.py` | 6 | `MKVFile` API, context manager, `duration`, `title`, filtering by type (planned) |
| `test_demux.py` | 8 | Frame iteration, `BlockData` types and timestamps (planned) |
| `test_cues.py` | 9 | Cue index lookup correctness (planned) |
| `test_seek.py` | 9 | Seek-to-timestamp accuracy (planned) |
| `test_chapters.py` | 10 | Chapter title and timestamp extraction (planned) |
| `test_tags.py` | 10 | Tag key/value extraction (planned) |
| `test_attachments.py` | 10 | Attachment name, MIME type, raw bytes (planned) |
| `test_integration.py` | 11 | Full pipeline over all 8 test files (planned) |

---

## Running Tests

```bash
# All tests:
uv run pytest tests/ -v

# Single phase:
uv run pytest tests/test_smoke.py -v
uv run pytest tests/test_errors.py tests/test_header.py tests/test_info.py -v

# With coverage (if pytest-cov added later):
uv run pytest tests/ --cov=pytroska --cov-report=term-missing

# Skip network (use pre-downloaded fixtures):
PYTROSKA_FIXTURES_DIR=/path/to/cached uv run pytest tests/ -v
```

pytest is configured in `pyproject.toml`:

```toml
[tool.pytest.ini_options]
testpaths = ["tests"]
```

---

## FAQ

**Q: How do I run tests without network access?**
A: Set `PYTROSKA_FIXTURES_DIR` to a directory that already contains `test1.mkv`–`test8.mkv`. The fixture code will detect the files exist, verify SHA-256, and skip download.

**Q: A test file hash mismatch was detected. What do I do?**
A: The official Matroska test file may have been updated. Delete `tests/fixtures/.hashes/testN.sha256` to accept the new version, then re-run the test.

**Q: Why does `test_errors.py` check `__module__`?**
A: PyO3 `create_exception!` macros set `__module__` to the string passed as the first argument. Tests verify it equals `'pytroska._pytroska_core'` (fully qualified) so that pickle round-trips work correctly — Python uses `__module__` + `__qualname__` to locate the class for deserialization.

---

## Related File List

- `/Users/Ninzero/Documents/python_program/pytroska/tests/conftest.py` — fixtures and test file downloads (Phase 3 complete)
- `/Users/Ninzero/Documents/python_program/pytroska/tests/test_smoke.py` — Phase 1 smoke tests
- `/Users/Ninzero/Documents/python_program/pytroska/tests/test_errors.py` — Phase 2 exception tests
- `/Users/Ninzero/Documents/python_program/pytroska/tests/test_header.py` — Phase 3 header tests
- `/Users/Ninzero/Documents/python_program/pytroska/tests/test_info.py` — Phase 4 SegmentInfo tests
- `/Users/Ninzero/Documents/python_program/pytroska/tests/fixtures/` — downloaded Matroska test MKV files (auto-created on first run)
- `/Users/Ninzero/Documents/python_program/pytroska/pyproject.toml` — pytest configuration

---

## Changelog

| Date | Description |
| ---- | ----------- |
| 2026-03-08 | Updated by architecture scan. Phases 2–4 complete: `conftest.py` (session fixtures + SHA-256 download/verify), `test_errors.py` (hierarchy + pickle), `test_header.py` (fields + error paths), `test_info.py` (SegmentInfo fields + timecode scale variants). Phase 5–11 test files documented as planned. |
| 2026-03-02 | Module CLAUDE.md created. Phase 1 state: `test_smoke.py` present. All other test files planned per phase. |
