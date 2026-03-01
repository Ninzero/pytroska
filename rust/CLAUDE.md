# Rust Core Module — CLAUDE.md

[Root](../../CLAUDE.md) > **rust/**

---

## Module Responsibility

The `rust/` directory contains the Rust source code for the `_pytroska_core` extension module. This layer is responsible for all performance-critical work: EBML binary parsing, file I/O, Matroska element extraction, lacing decode, timecode math, and cue-based seeking. It exposes Python classes and functions via PyO3.

The compiled output is a `cdylib` named `_pytroska_core` (a `.so` / `.pyd` file) placed inside the `python/pytroska/` package directory by `maturin develop`.

---

## Entry Point and Startup

`rust/lib.rs` is the PyO3 module root. The `#[pymodule]` function `_pytroska_core` is the entry point Maturin wires to Python. Every submodule and pyfunction is registered here.

Current state (Phase 1):

```rust
// rust/lib.rs
use pyo3::prelude::*;

#[pyfunction]
fn core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn _pytroska_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    Ok(())
}
```

Planned additions per phase:

| Phase | File | Registered items |
|-------|------|-----------------|
| 2 | `errors.rs` | `PytroskaError`, `ParseError`, `CorruptedError`, `UnsupportedError` exception classes |
| 3 | `header.rs` | `EbmlHeader` pyclass, `parse_ebml_header(path)` pyfunction |
| 4 | `reader.rs`, `info.rs` | `SegmentInfo` pyclass, `parse_segment_info(path)` pyfunction |
| 5 | `tracks.rs` | `TrackInfo`, `VideoSettings`, `AudioSettings` pyclasses, `parse_tracks(path)` pyfunction |
| 7 | `utils/lacing.rs`, `utils/timecode.rs` | Lacing and timecode utility functions |
| 8 | `cluster.rs`, `demux.rs` | `BlockData` pyclass, `iter_frames(path, track_number)` pyfunction |
| 9 | `cues.rs` | `CuePoint` pyclass, `seek_to_timestamp(path, ns)` pyfunction |
| 10 | `chapters.rs`, `tags.rs`, `attachments.rs` | Respective pyclasses and parse functions |

---

## External Interfaces (PyO3 API)

All public Python-facing items live in the `_pytroska_core` namespace. Python code imports them as:

```python
from pytroska._pytroska_core import core_version
```

Type stubs for every exported item are maintained in `python/pytroska/_pytroska_core.pyi`.

---

## Key Dependencies

```toml
# Cargo.toml
pyo3 = { version = "0.28.2", features = ["extension-module"] }
thiserror = "2.0.18"
webm-iterable = "0.6.4"
```

- **pyo3**: Rust-Python FFI. All `#[pyclass]`, `#[pyfunction]`, `#[pymodule]` macros come from here.
- **thiserror**: Derive macros for the `PytroskaError` enum in `errors.rs`.
- **webm-iterable**: Provides `TagIterator<_, MatroskaSpec>` — an iterator over EBML elements with full Matroska schema awareness. This is the primary parsing primitive used in every parser module.

---

## Planned Module Files

| File | Responsibility |
|------|---------------|
| `lib.rs` | PyO3 module entry; registers all exported symbols |
| `errors.rs` | `PytroskaError` enum with variants: `Io`, `Parse`, `InvalidHeader`, `UnsupportedDocType`, `Unsupported`, `Corrupted`; `From<PytroskaError> for PyErr` |
| `header.rs` | `EbmlHeader` pyclass (version, read_version, max_id_length, max_size_length, doc_type, doc_type_version, doc_type_read_version); `parse_ebml_header(path)` |
| `reader.rs` | `MkvReader { reader: BufReader<File>, header: EbmlHeader, info: Option<SegmentInfo>, segment_offset: u64 }`; `open(path)` and `parse_metadata()` methods |
| `info.rs` | `SegmentInfo` pyclass (duration_ns, duration, timecode_scale, title, muxing_app, writing_app, date_utc, segment_uid); `extract_info(tags)` |
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

```rust
// rust/errors.rs (Phase 2)
#[derive(Debug, thiserror::Error)]
pub enum PytroskaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error at position {position}: {message}")]
    Parse { position: u64, message: String },
    #[error("Invalid EBML header: {0}")]
    InvalidHeader(String),
    #[error("Unsupported DocType: expected 'matroska' or 'webm', got '{0}'")]
    UnsupportedDocType(String),
    #[error("Unsupported feature: {0}")]
    Unsupported(String),
    #[error("Corrupted data: {0}")]
    Corrupted(String),
}

impl From<PytroskaError> for PyErr {
    // Io -> PyIOError
    // Parse / InvalidHeader -> ParseError
    // Unsupported / UnsupportedDocType -> UnsupportedError
    // Corrupted -> CorruptedError
}
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

- `/Users/Ninzero/Documents/python_program/pytroska/rust/lib.rs` — PyO3 module entry (exists, Phase 1)
- `/Users/Ninzero/Documents/python_program/pytroska/Cargo.toml` — Rust package + dependency definitions
- `/Users/Ninzero/Documents/python_program/pytroska/python/pytroska/_pytroska_core.pyi` — Python type stubs (must stay in sync)
- `/Users/Ninzero/Documents/python_program/pytroska/.plans/pytroska-implement-phases.md` — Detailed per-phase implementation plan

---

## Changelog

| Date | Description |
|------|-------------|
| 2026-03-02 | Module CLAUDE.md created. Phase 1 state: `lib.rs` with single `core_version()` function. Phases 2–10 documented as planned. |
