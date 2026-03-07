# Python Package — CLAUDE.md

[Root](../../CLAUDE.md) > [python/](../) > **pytroska/**

---

## Module Responsibility

`python/pytroska/` is the user-facing Python package. It provides:

- A Pythonic, type-annotated API over the Rust extension module `_pytroska_core`
- The `MKVFile` high-level class with context-manager support (Phase 6)
- Track, Chapter, Tag, Attachment, Cue Python wrapper classes (Phase 6–10)
- A clean exception hierarchy re-exported from the Rust layer (`exceptions.py`)
- PEP 561 compliance (`py.typed` marker) for downstream type checking

The package is installed in editable mode via `maturin develop --uv`. The Rust extension `.so` is placed directly inside this directory as `_pytroska_core.<platform>.so`.

---

## Entry Point

`python/pytroska/__init__.py` is the public API surface.

Current state (Phase 4 complete):

```python
from pytroska.exceptions import ParseError as ParseError
from pytroska.exceptions import PytroskaError as PytroskaError
from pytroska.exceptions import CorruptedError as CorruptedError
from pytroska.exceptions import UnsupportedError as UnsupportedError
from pytroska._pytroska_core import SegmentInfo as SegmentInfo
from pytroska._pytroska_core import core_version as core_version
from pytroska._pytroska_core import parse_segment_info as parse_segment_info

__version__: str = '0.1.0'

__all__ = [
    'CorruptedError', 'ParseError', 'PytroskaError', 'SegmentInfo',
    'UnsupportedError', '__version__', 'core_version', 'parse_segment_info',
]
```

Note: `EbmlHeader` and `parse_ebml_header` are implemented in Rust (Phase 3) and accessible via `pytroska._pytroska_core` directly, but are not yet re-exported in `__init__.py`. Phase 6 should add them if needed for the `MKVFile` public API.

Planned additions per phase:

| Phase | New exports |
|-------|------------|
| 5 | Rust-side `TrackInfo`, `VideoSettings`, `AudioSettings` via `_pytroska_core.pyi` |
| 6 | `MKVFile`, `Track`, `VideoTrack`, `AudioTrack` from `file.py` and `tracks.py`; `TrackType` from `types.py` |
| 9 | `Cue` from `cues.py` |
| 10 | `Chapter`, `Tag`, `Attachment` from `chapters.py`, `tags.py`, `attachments.py` |

---

## Implemented Files

### `exceptions.py` (Phase 2 complete)

Re-exports the four exception classes from `pytroska._pytroska_core`:

```python
from pytroska._pytroska_core import ParseError as ParseError
from pytroska._pytroska_core import PytroskaError as PytroskaError
from pytroska._pytroska_core import CorruptedError as CorruptedError
from pytroska._pytroska_core import UnsupportedError as UnsupportedError
```

### `_pytroska_core.pyi` (Phase 4 complete)

Type stubs for the Rust extension. Current exported symbols:

```python
def core_version() -> str: ...

class PytroskaError(Exception): ...
class ParseError(PytroskaError): ...
class CorruptedError(PytroskaError): ...
class UnsupportedError(PytroskaError): ...

class EbmlHeader:
    version: int; read_version: int; max_id_length: int; max_size_length: int
    doc_type: str; doc_type_version: int; doc_type_read_version: int
    def __repr__(self) -> str: ...

def parse_ebml_header(path: str | PathLike[str]) -> EbmlHeader: ...

class SegmentInfo:
    duration_raw: float | None; timecode_scale: int; title: str | None
    muxing_app: str; writing_app: str; date_utc_raw: int | None
    segment_uid: str | None
    def __repr__(self) -> str: ...

def parse_segment_info(path: str | PathLike[str]) -> SegmentInfo: ...
```

---

## Planned File Structure

```
python/pytroska/
├── __init__.py           # Public API, re-exports (Phase 4 complete)
├── py.typed              # PEP 561 marker (Phase 1 complete)
├── _pytroska_core.pyi    # Type stubs for Rust extension (Phase 4 complete)
├── exceptions.py         # Exception re-exports from _pytroska_core (Phase 2 complete)
├── types.py              # TrackType IntEnum and shared types (Phase 5)
├── tracks.py             # Track / VideoTrack / AudioTrack wrappers (Phase 6)
├── file.py               # MKVFile high-level class (Phase 6)
├── cues.py               # Cue wrapper (Phase 9)
├── chapters.py           # Chapter wrapper (Phase 10)
├── tags.py               # Tag wrapper (Phase 10)
├── attachments.py        # Attachment wrapper (Phase 10)
└── utils.py              # verify_mkv_file, get_media_info helpers (Phase 6+)
```

---

## Public Interface

### Phase 4 current API

```python
import pytroska

pytroska.__version__        # '0.1.0'
pytroska.core_version()     # Rust crate version string, e.g. '0.1.0'

info = pytroska.parse_segment_info('movie.mkv')
info.duration_raw           # float | None (in timecode units)
info.timecode_scale         # int, default 1_000_000 nanoseconds
info.title                  # str | None
info.muxing_app             # str
info.writing_app            # str
info.date_utc_raw           # int | None (nanoseconds since 2001-01-01)
info.segment_uid            # str | None (32-char hex, e.g. 'a3f2...')

# Also accessible directly:
from pytroska._pytroska_core import parse_ebml_header, EbmlHeader
h = parse_ebml_header('movie.mkv')
h.doc_type                  # 'matroska' or 'webm'
```

### Exception hierarchy

```
Exception
  PytroskaError
    ParseError        -- malformed EBML, invalid header, missing Info element
    CorruptedError    -- data integrity failure
    UnsupportedError  -- unknown DocType or unimplemented feature
  OSError / FileNotFoundError  -- standard Python I/O errors (from Rust Io variant)
```

### Phase 6 target API

```python
from pytroska import MKVFile, TrackType

with MKVFile('movie.mkv') as mkv:
    print(mkv.duration)           # float | None, seconds
    print(mkv.title)              # str | None
    for track in mkv.tracks:
        print(track.number, track.codec_id, track.track_type)
    videos = mkv.get_tracks_by_type(TrackType.VIDEO)
    audios = mkv.get_tracks_by_type(TrackType.AUDIO)
```

---

## Configuration (`pyproject.toml` highlights)

```toml
[tool.maturin]
features = ["pyo3/extension-module"]
python-source = "python"
module-name = "pytroska._pytroska_core"

[tool.ruff]
target-version = "py311"
line-length = 88

[tool.ruff.lint]
select = ["E", "F", "N", "W", "A", "B", "C4", "PT", "UP", "ANN", "SIM", "RUF"]

[tool.pyright]
pythonVersion = "3.11"
typeCheckingMode = "strict"
include = ["python/pytroska"]
```

Ruff lint rules: E, F, N, W, A, B, C4, PT, UP, ANN, SIM, RUF. Single-quoted strings; Numpy docstrings.

Known config issue: `known_first_party = ["pytroska "]` in `[tool.isort]` has a trailing space — should be `["pytroska"]`.

---

## Quality Checks

```bash
uv run ruff check python/ tests/
uv run ruff format --check python/ tests/
uv run isort --check python/ tests/
uv run pyright python/
```

---

## FAQ

**Q: Why is the Rust `.so` inside `python/pytroska/` directly?**
A: `maturin` places the compiled extension at the `module-name` path. With `module-name = "pytroska._pytroska_core"` and `python-source = "python"`, the output is `python/pytroska/_pytroska_core.<platform>.so`.

**Q: How do I add a new Rust function to the Python API?**
A: (1) Implement the function/struct in the relevant `rust/*.rs` file, (2) register it in `rust/lib.rs`, (3) add the stub in `python/pytroska/_pytroska_core.pyi`, (4) re-export in `python/pytroska/__init__.py` if it is public API, (5) run `uv run maturin develop --uv`.

**Q: Why is `EbmlHeader` not in `__init__.py`?**
A: Currently it is only used by test code via `from pytroska._pytroska_core import EbmlHeader`. It will be added to public API in Phase 6 when `MKVFile` exposes a `.header` property.

**Q: What does `duration_raw` mean?**
A: It is the raw `Duration` field from the Matroska Info element, in units of `timecode_scale` nanoseconds. To get seconds: `duration_seconds = duration_raw * timecode_scale / 1e9`.

---

## Related File List

- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/__init__.py` — public API entry
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/_pytroska_core.pyi` — Rust extension type stubs
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/exceptions.py` — exception re-exports
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/py.typed` — PEP 561 marker
- `/Users/Ninzero/Documents/python_program/pytroska/pyproject.toml` — build, lint, and type-check configuration
- `/Users/Ninzero/Documents/python_program/pytroska/rust/CLAUDE.md` — Rust layer docs

---

## Changelog

| Date | Description |
|------|-------------|
| 2026-03-08 | Updated by architecture scan. Phase 2 complete: `exceptions.py` added (re-exports all 4 exception classes). Phase 4 complete: `__init__.py` now exports `SegmentInfo`, `parse_segment_info`, all exceptions; `_pytroska_core.pyi` updated with `EbmlHeader`, `SegmentInfo`, `parse_ebml_header`, `parse_segment_info`. `EbmlHeader` noted as not yet in public `__init__.py`. isort config trailing-space bug documented. |
| 2026-03-02 | Module CLAUDE.md created. Phase 1 state: `__init__.py`, `_pytroska_core.pyi`, `py.typed`. Phases 2–11 documented as planned. |
