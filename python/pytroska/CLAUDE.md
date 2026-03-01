# Python Package — CLAUDE.md

[Root](../../../CLAUDE.md) > [python/](../../) > **pytroska/**

---

## Module Responsibility

`python/pytroska/` is the user-facing Python package. It provides:

- A Pythonic, type-annotated API over the Rust extension module `_pytroska_core`
- The `MKVFile` high-level class with context-manager support
- Track, Chapter, Tag, Attachment, Cue Python wrapper classes
- A clean exception hierarchy re-exported from the Rust layer
- PEP 561 compliance (`py.typed` marker) for downstream type checking

The package is installed in editable mode via `maturin develop --uv`. The Rust extension `.so` is placed directly inside this directory as `_pytroska_core.<platform>.so`.

---

## Entry Point

`python/pytroska/__init__.py` is the public API surface.

Current state (Phase 1):

```python
"""Pytroska - High-performance MKV file processing library."""

from pytroska._pytroska_core import core_version as core_version

__version__: str = "0.1.0"

__all__ = ["__version__", "core_version"]
```

Planned additions per phase:

| Phase | New exports |
|-------|------------|
| 2 | `PytroskaError`, `ParseError`, `CorruptedError`, `UnsupportedError` from `exceptions.py` |
| 4 | `TrackType` from `types.py` |
| 5 | Rust-side `TrackInfo`, `VideoSettings`, `AudioSettings` via `_core.pyi` |
| 6 | `MKVFile`, `Track`, `VideoTrack`, `AudioTrack` from `file.py` and `tracks.py` |
| 9 | `Cue` from `cues.py` |
| 10 | `Chapter`, `Tag`, `Attachment` from `chapters.py`, `tags.py`, `attachments.py` |

---

## Planned File Structure

```
python/pytroska/
├── __init__.py           # Public API, re-exports (Phase 1 exists)
├── py.typed              # PEP 561 marker (Phase 1 exists)
├── _pytroska_core.pyi    # Type stubs for Rust extension (Phase 1 exists)
├── exceptions.py         # Exception re-exports from _pytroska_core (Phase 2)
├── types.py              # TrackType IntEnum and shared types (Phase 4)
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

### Phase 1 (current)

```python
import pytroska

pytroska.__version__   # "0.1.0"
pytroska.core_version()  # returns Rust crate version string, e.g. "0.1.0"
```

### Phase 6 target API

```python
from pytroska import MKVFile, TrackType

with MKVFile("movie.mkv") as mkv:
    print(mkv.duration)           # float | None, seconds
    print(mkv.title)              # str | None
    for track in mkv.tracks:
        print(track.number, track.codec_id, track.track_type)
    videos = mkv.get_tracks_by_type(TrackType.VIDEO)
    audios = mkv.get_tracks_by_type(TrackType.AUDIO)
```

### Exception hierarchy (Phase 2)

```
Exception
  PytroskaError
    ParseError        — malformed EBML, invalid header
    CorruptedError    — data integrity failure
    UnsupportedError  — unknown DocType or unimplemented feature
  IOError             — standard Python IOError for file-not-found etc.
```

---

## Key Type Definitions

### `TrackType` (Phase 4, `types.py`)

```python
class TrackType(enum.IntEnum):
    VIDEO    = 1
    AUDIO    = 2
    COMPLEX  = 3
    LOGO     = 16
    SUBTITLE = 17
    BUTTONS  = 18
    CONTROL  = 32
    METADATA = 33
```

### `MKVFile` (Phase 6, `file.py`)

```python
class MKVFile:
    def __init__(self, path: str | Path) -> None: ...
    @property
    def header(self) -> EbmlHeader: ...
    @property
    def info(self) -> SegmentInfo: ...
    @property
    def duration(self) -> float | None: ...
    @property
    def title(self) -> str | None: ...
    @property
    def tracks(self) -> Sequence[Track]: ...
    def get_track_by_number(self, number: int) -> Track | None: ...
    def get_tracks_by_type(self, track_type: TrackType) -> list[Track]: ...
    def __enter__(self) -> MKVFile: ...
    def __exit__(self, *args: object) -> None: ...
    def close(self) -> None: ...
```

### `Track` (Phase 6, `tracks.py`)

```python
class Track:
    @property
    def number(self) -> int: ...
    @property
    def track_type(self) -> TrackType: ...
    @property
    def codec_id(self) -> str: ...
    @property
    def language(self) -> str | None: ...
    @property
    def name(self) -> str | None: ...
    @property
    def video(self) -> VideoTrack | None: ...
    @property
    def audio(self) -> AudioTrack | None: ...
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

[tool.pyright]
pythonVersion = "3.11"
typeCheckingMode = "strict"
include = ["python/pytroska"]
```

Ruff lint rules enabled: `E, F, N, W, A, B, C4, PT, UP, ANN, SIM, RUF`.
Single-quoted strings; docstrings follow Numpy convention.

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

**Q: Why does `isort` use `known_first_party = ["tg_chat_bot"]` in `pyproject.toml`?**
A: This appears to be a copy-paste artifact from the project template. It should be changed to `["pytroska"]` when `isort` configuration is finalized.

---

## Related File List

- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/__init__.py` — public API entry
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/_pytroska_core.pyi` — Rust extension type stubs
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/py.typed` — PEP 561 marker
- `/Users/Ninzero/Documents/python_program/pytroska/pyproject.toml` — build, lint, and type-check configuration
- `/Users/Ninzero/Documents/python_program/pytroska/rust/CLAUDE.md` — Rust layer docs (the module this package wraps)

---

## Changelog

| Date | Description |
|------|-------------|
| 2026-03-02 | Module CLAUDE.md created. Current state: Phase 1 complete (`__init__.py`, `_pytroska_core.pyi`, `py.typed`). Phases 2–11 documented as planned. |
