# Rust Core Module — CLAUDE.md

[Root](../CLAUDE.md) > **rust/**

---

## Module Responsibility

The `rust/` directory contains the Rust source code for the `_pytroska_core` extension module. This layer is responsible for all performance-critical work: EBML binary parsing, file I/O, Matroska element extraction, lacing decode, timecode math, and cue-based seeking. It exposes Python classes and functions via PyO3.

The compiled output is a `cdylib` named `_pytroska_core` (a `.so` / `.pyd` file) placed inside the `python/pytroska/` package directory by `maturin develop`.

---

## Entry Point and Startup

`rust/lib.rs` is the PyO3 module root. The `#[pymodule]` function `_pytroska_core` is the entry point Maturin wires to Python. Every submodule and pyfunction is registered here.

Current state (Phase 4 complete):

```rust
// rust/lib.rs
mod errors;
mod header;
mod info;
mod reader;

use errors::{CorruptedError, ParseError, PytroskaError, UnsupportedError};
use header::{EbmlHeader, parse_ebml_header};
use info::SegmentInfo;
use reader::MkvReader;

#[pyfunction]
fn core_version() -> &'static str { env!("CARGO_PKG_VERSION") }

#[pyfunction]
fn parse_segment_info(path: PathBuf) -> PyResult<SegmentInfo> {
    let reader = MkvReader::open(&path)?;
    Ok(reader.info)
}

#[pymodule]
fn _pytroska_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Exceptions (Phase 2)
    m.add("PytroskaError", py.get_type::<PytroskaError>())?;
    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("CorruptedError", py.get_type::<CorruptedError>())?;
    m.add("UnsupportedError", py.get_type::<UnsupportedError>())?;
    // Phase 1
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    // Phase 3
    m.add_class::<EbmlHeader>()?;
    m.add_function(wrap_pyfunction!(parse_ebml_header, m)?)?;
    // Phase 4
    m.add_class::<SegmentInfo>()?;
    m.add_function(wrap_pyfunction!(parse_segment_info, m)?)?;
    Ok(())
}
```

Planned additions per phase:

| Phase | File | Registered items |
| ----- | ---- | --------------- |
| 5 | `tracks.rs` | `TrackInfo`, `VideoSettings`, `AudioSettings` pyclasses, `parse_tracks(path)` pyfunction |
| 7 | `utils/lacing.rs`, `utils/timecode.rs` | Lacing and timecode utility functions |
| 8 | `cluster.rs`, `demux.rs` | `BlockData` pyclass, `iter_frames(path, track_number)` pyfunction |
| 9 | `cues.rs` | `CuePoint` pyclass, `seek_to_timestamp(path, ns)` pyfunction |
| 10 | `chapters.rs`, `tags.rs`, `attachments.rs` | Respective pyclasses and parse functions |

---

## Implemented Files

### `errors.rs` (Phase 2 complete)

Defines `PytroskaRustError` (the internal Rust error enum, using `thiserror`) and four Python exception classes via `create_exception!`. The `From<PytroskaRustError> for PyErr` impl maps variants:

- `Io` -> `PyErr::from(io::Error)` (becomes Python `OSError` / `FileNotFoundError`)
- `Parse` + `InvalidHeader` -> `ParseError`
- `UnsupportedDocType` + `Unsupported` -> `UnsupportedError`
- `Corrupted` -> `CorruptedError`

`map_tag_iterator_error()` is a helper converting `webm_iterable::errors::TagIteratorError` to `PytroskaRustError`.

### `header.rs` (Phase 3 complete)

`EbmlHeader` pyclass fields: `version`, `read_version`, `max_id_length`, `max_size_length`, `doc_type`, `doc_type_version`, `doc_type_read_version`. All `u64` on the Rust side, exposed as `int` in Python.

- `parse_header_from_iter<R: Read>(iter, path_display)` — internal; used by `MkvReader::open()` and `parse_ebml_header()`
- `parse_ebml_header(path: PathBuf) -> PyResult<EbmlHeader>` — public pyfunction

DocType validation: rejects anything other than `"matroska"` or `"webm"` with `UnsupportedError`.

### `reader.rs` (Phase 4 complete)

`MkvReader { header: EbmlHeader, info: SegmentInfo }` — internal struct (not a pyclass).

`MkvReader::open(path)` creates a `WebmIterator` with `tags_to_buffer = [MatroskaSpec::Info(Master::Start)]` so `Info` is returned as `Master::Full(children)`. Then calls `parse_header_from_iter` then `extract_info` sequentially.

**Important Phase 5 constraint** (documented in source): this sequential pattern breaks if `Tracks` appears before `Info` in the file. Phase 5 must replace the sequential calls with a single-pass loop. See the `TODO(Phase 5)` comment in `reader.rs` for the exact refactor pattern.

### `info.rs` (Phase 4 complete)

`SegmentInfo` pyclass fields: `duration_raw: Option<f64>`, `timecode_scale: u64`, `title: Option<String>`, `muxing_app: String`, `writing_app: String`, `date_utc_raw: Option<i64>`, `segment_uid: Option<String>`.

`parse_info_children(&[MatroskaSpec])` — pure function over buffered children; default `timecode_scale = 1_000_000`. `SegmentUID` validated to exactly 16 bytes; invalid lengths emit a `tracing::warn!` and set `segment_uid = None`.

`extract_info<R: Read>(iter, path_display)` — loops until `Info(Master::Full)` found; errors on `Cluster` hit (missing Info) or iterator exhausted.

---

## External Interfaces (PyO3 API)

All public Python-facing items live in the `_pytroska_core` namespace:

```python
from pytroska._pytroska_core import (
    core_version,          # () -> str
    parse_ebml_header,     # (path) -> EbmlHeader
    parse_segment_info,    # (path) -> SegmentInfo
    EbmlHeader,            # class
    SegmentInfo,           # class
    PytroskaError,         # exception
    ParseError,            # exception
    CorruptedError,        # exception
    UnsupportedError,      # exception
)
```

Type stubs for every exported item are maintained in `python/pytroska/_pytroska_core.pyi`.

---

## Key Dependencies

```toml
# Cargo.toml
[dependencies]
pyo3        = { version = "0.28.2", features = ["extension-module"] }
thiserror   = "2.0.18"
tracing     = "0.1.44"
webm-iterable = "0.6.4"
```

- **pyo3**: Rust-Python FFI. All `#[pyclass]`, `#[pyfunction]`, `#[pymodule]` macros come from here.
- **thiserror**: Derive macros for the `PytroskaRustError` enum.
- **tracing**: Structured logging (currently used in `info.rs` for SegmentUID length warnings).
- **webm-iterable**: Provides `WebmIterator<R, MatroskaSpec>` — iterator over EBML elements with full Matroska schema awareness. Primary parsing primitive used in every parser module.

---

## Planned Module Files

| File | Responsibility |
| ---- | -------------- |
| `tracks.rs` | `TrackInfo`, `VideoSettings`, `AudioSettings` pyclasses; `extract_tracks(tags)` |
| `cluster.rs` | Cluster and SimpleBlock/Block element parsing |
| `demux.rs` | Iterator over decoded `BlockData` structs |
| `cues.rs` | `CuePoint` pyclass; seek-by-timestamp using Cues index |
| `chapters.rs` | `ChapterAtom` extraction |
| `tags.rs` | `TagEntry` extraction |
| `attachments.rs` | `Attachment` extraction (name, MIME type, raw bytes) |
| `utils/mod.rs` | Utils submodule declaration |
| `utils/lacing.rs` | `decode_xiph_lacing`, `decode_ebml_lacing`, `decode_fixed_lacing` |
| `utils/timecode.rs` | `ns_to_timestamp(ns) -> String`, `timestamp_to_ns(ts) -> Result<u64>` |

---

## Error Handling Pattern

```text
PytroskaRustError (Rust internal)
  Io(std::io::Error)          -> Python OSError / FileNotFoundError
  Parse { position, message } -> Python ParseError
  InvalidHeader(String)       -> Python ParseError
  UnsupportedDocType(String)  -> Python UnsupportedError
  Unsupported(String)         -> Python UnsupportedError
  Corrupted(String)           -> Python CorruptedError

TagIteratorError              -> map_tag_iterator_error() -> PytroskaRustError
  ReadError                   -> Io
  CorruptedFileData / CorruptedTagData -> Corrupted
  UnexpectedEOF               -> Parse
```

---

## Build

```bash
# Development (editable install):
uv run maturin develop --uv

# Release wheel:
uv run maturin build --release

# Rust-only checks:
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

The `[profile.dev]` in `Cargo.toml` sets `opt-level = 1` to improve iteration speed during development without full debug overhead.

---

## Related File List

- `/Users/Ninzero/Documents/python_program/pytroska/rust/lib.rs` — PyO3 module entry (Phase 4 complete)
- `/Users/Ninzero/Documents/python_program/pytroska/rust/errors.rs` — error types and Python exception creation (Phase 2 complete)
- `/Users/Ninzero/Documents/python_program/pytroska/rust/header.rs` — EbmlHeader (Phase 3 complete)
- `/Users/Ninzero/Documents/python_program/pytroska/rust/reader.rs` — MkvReader (Phase 4 complete)
- `/Users/Ninzero/Documents/python_program/pytroska/rust/info.rs` — SegmentInfo (Phase 4 complete)
- `/Users/Ninzero/Documents/python_program/pytroska/Cargo.toml` — Rust package + dependency definitions
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/_pytroska_core.pyi` — Python type stubs (must stay in sync)
- `/Users/Ninzero/Documents/python_program/pytroska/.plans/pytroska-implement-phases.md` — Detailed per-phase implementation plan

---

## Changelog

| Date | Description |
| ---- | ----------- |
| 2026-03-08 | Updated by architecture scan. Phases 2–4 complete: `errors.rs` (PytroskaRustError enum, 4 Python exception classes, map_tag_iterator_error), `header.rs` (EbmlHeader, parse_ebml_header), `reader.rs` (MkvReader::open with buffered Info iteration), `info.rs` (SegmentInfo, parse_info_children, extract_info). `tracing` dependency added. Phase 5 ordering constraint documented. |
| 2026-03-02 | Module CLAUDE.md created. Phase 1 state: `lib.rs` with single `core_version()` function. Phases 2–10 documented as planned. |
