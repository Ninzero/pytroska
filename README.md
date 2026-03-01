# Pytroska

> High-performance MKV (Matroska) file processing library for Python — powered by Rust.

**Status: Pre-Alpha / Active Development** — Core parsing is being built phase by phase. The public API is not stable yet.

---

## Why Pytroska?

The Python ecosystem lacks a native, high-performance MKV library:

| Library | Approach | Problem |
| ------- | -------- | ------- |
| `pymkv` | Wraps `mkvmerge` CLI | Requires external tool; no direct file access |
| `mkvparse` | Pure Python | Last updated 2022; no seeking, no Cues |
| `python-matroska` | Pure Python | 4 stars; low activity |
| **pytroska** | Rust + Python | Native binary parsing; no external dependencies |

Pytroska parses EBML/Matroska binaries directly in Rust and exposes a Pythonic, fully type-annotated API — no `mkvmerge`, no subprocess, no compromises.

---

## Architecture

```text
┌────────────────────────────────────────────────────────────────┐
│                        Python Layer                            │
│                                                                │
│  MKVFile  │  Track  │  Chapter  │  Tag  │  Attachment  │  Cue  │
│                                                                │
│            Pythonic API  ·  type-annotated  ·  PEP 561         │
└───────────────────────────────┬────────────────────────────────┘
                                │
                        PyO3 / maturin FFI
                                │
┌───────────────────────────────▼────────────────────────────────┐
│                           Rust Layer                           │
│                                                                │
│   EBML Parser         Segment                 I/O              │
│   webm-iterable   ·   info / tracks     ·     BufReader<File>  │
│   TagIterator         cluster / cues          seeking          │
│   MatroskaSpec        chapters / tags         lacing           │
│                       attachments             timecodes        │
└────────────────────────────────────────────────────────────────┘
```

**Key design decisions:**

| Decision | Choice |
| -------- | ------ |
| EBML parsing | `webm-iterable` crate (`TagIterator<_, MatroskaSpec>`) |
| Python binding | PyO3 + maturin |
| Feature priority | Read-first; write support in later phases |
| Python version | >= 3.11 |
| Dependency manager | `uv` |

---

## Installation

### Prerequisites

- Python >= 3.11
- Rust toolchain (stable) with `cargo` in `$PATH`
- [`uv`](https://github.com/astral-sh/uv) package manager

### From source (development)

```bash
git clone https://github.com/your-org/pytroska.git
cd pytroska

# Install Python dependencies and build the Rust extension
uv sync --all-groups --no-install-project
uv run maturin develop --uv
```

### Verify the build

```python
import pytroska

print(pytroska.__version__)    # "0.1.0"
print(pytroska.core_version()) # Rust core version string
```

> **Note:** PyPI releases will be available once the library reaches a stable API milestone (v0.6+).

---

## Quick Start

### What works today (v0.1 — Phase 1)

The Rust/Python bridge is in place and verified. You can confirm the build is healthy:

```python
import pytroska

print(pytroska.__version__)    # "0.1.0"
print(pytroska.core_version()) # e.g. "0.1.0"
```

### What's coming (v0.6+ target API)

Once Phase 6 lands, the high-level Python API will look like this:

```python
from pytroska import MKVFile, TrackType

# Context manager — file is opened and closed cleanly
with MKVFile("movie.mkv") as mkv:
    print(mkv.title)                        # str | None
    print(f"Duration: {mkv.duration:.1f}s") # float | None

    # Iterate all tracks
    for track in mkv.tracks:
        print(f"  [{track.number}] {track.codec_id} ({track.track_type.name})")
        if track.video:
            print(f"    {track.video.pixel_width}x{track.video.pixel_height}")
        if track.audio:
            print(f"    {track.audio.channels}ch @ {track.audio.sampling_frequency}Hz")

    # Filter by type
    videos = mkv.get_tracks_by_type(TrackType.VIDEO)
    audios = mkv.get_tracks_by_type(TrackType.AUDIO)
    subs   = mkv.get_tracks_by_type(TrackType.SUBTITLE)
```

Chapters, tags, and attachments will follow in Phase 10:

```python
with MKVFile("movie.mkv") as mkv:
    for chapter in mkv.chapters:
        print(f"{chapter.start_time:.3f}s — {chapter.title}")

    for tag in mkv.tags:
        print(tag.name, tag.value)

    for att in mkv.attachments:
        att.save_to("./output/")
```

---

## Roadmap

Development follows a strict phase order — each phase depends on the one before it.

| Phase | Status | What's built | Key API / Types |
| ----- | ------ | ------------ | --------------- |
| **1** | ✅ Done | Project scaffold — Cargo, pyproject, PyO3 bridge | `core_version()`, `__version__` |
| **2** | ⬜ Planned | Error types in Rust; Python exception hierarchy | `PytroskaError`, `ParseError`, `CorruptedError`, `UnsupportedError` |
| **3** | ⬜ Planned | Test infrastructure; EBML Header parsing | `EbmlHeader` (doc_type, version, read_version) |
| **4** | ⬜ Planned | `MkvReader` core struct; Segment Info extraction | `SegmentInfo` (duration, title, muxing_app, timecode_scale) |
| **5** | ⬜ Planned | Track parsing in Rust | `TrackInfo`, `VideoSettings` (resolution, FPS), `AudioSettings` (channels, sample rate) |
| **6** | ⬜ Planned | High-level Python API | `MKVFile`, `Track`, `VideoTrack`, `AudioTrack`, `TrackType` |
| **7** | ⬜ Planned | Low-level decode utilities | Lacing (Xiph / EBML / fixed-size), nanosecond timecode conversion |
| **8** | ⬜ Planned | Cluster / Block parsing; demux iterator | Frame-level iteration, decoded packet structure |
| **9** | ⬜ Planned | Cues index + seeking | `Cue`, seek-by-timestamp, random access |
| **10** | ⬜ Planned | Chapters, Tags, Attachments | `Chapter`, `Tag`, `Attachment`, save-to-disk |
| **11** | ⬜ Planned | Integration tests + API stabilization | Full pipeline over all 8 official Matroska test files |

---

## Development

### Daily cycle

```bash
# After any Rust change, rebuild the extension:
uv run maturin develop --uv

# Rust quality checks:
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Python quality checks:
uv run ruff check python/ tests/
uv run ruff format --check python/ tests/
uv run isort --check python/ tests/
uv run pyright python/

# Run all tests:
uv run pytest tests/ -v
```

### Build a release wheel

```bash
uv run maturin build --release
```

### Project structure

```text
pytroska/
├── Cargo.toml              # Rust package
├── pyproject.toml          # Python package + maturin + tool config
├── rust/                   # Rust source (EBML parser, reader, demux …)
│   └── lib.rs              # PyO3 module entry point
├── python/pytroska/        # Python package
│   ├── __init__.py         # Public API
│   └── _pytroska_core.pyi  # Type stubs for the Rust extension
└── tests/                  # pytest suite (fixtures auto-downloaded)
```

---

## Contributing

Contributions are welcome! A few things to know before diving in:

- Read the architecture overview in [`CLAUDE.md`](./CLAUDE.md) and the phase plan in [`.plans/pytroska-implement-phases.md`](./.plans/pytroska-implement-phases.md).
- Implement phases in order — later phases have hard dependencies on earlier ones.
- Every new Rust `pyclass` or `pyfunction` must be accompanied by an update to `python/pytroska/_pytroska_core.pyi`.
- Run `cargo clippy -- -D warnings` and `uv run pyright python/` before submitting a PR.

---

## License

MIT — see [`LICENSE`](./LICENSE).
