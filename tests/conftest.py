"""Test fixtures: download and integrity-verify official Matroska test files."""

from __future__ import annotations

import os
import hashlib
import urllib.request
from pathlib import Path

import pytest

_FIXTURES_DIR = Path(
    os.environ.get(
        'PYTROSKA_FIXTURES_DIR',
        str(Path(__file__).resolve().parent / 'fixtures'),
    )
)
_HASHES_DIR = _FIXTURES_DIR / '.hashes'
_BASE_URL = (
    'https://raw.githubusercontent.com/ietf-wg-cellar/'
    'matroska-test-files/master/test_files'
)


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as fh:
        while chunk := fh.read(1024 * 1024):
            h.update(chunk)
    return h.hexdigest()


def _download(n: int, dest: Path) -> None:
    url = f'{_BASE_URL}/test{n}.mkv'
    part = dest.with_suffix('.part')
    try:
        urllib.request.urlretrieve(url, str(part))
        part.replace(dest)
    except OSError as exc:
        part.unlink(missing_ok=True)
        pytest.fail(
            f'Failed to download test{n}.mkv: {exc}\n'
            f'URL: {url}\n'
            f'Ensure network access or manually place the file at {dest}'
        )


def _ensure_test_file(n: int) -> Path:
    _FIXTURES_DIR.mkdir(parents=True, exist_ok=True)
    _HASHES_DIR.mkdir(parents=True, exist_ok=True)

    path = _FIXTURES_DIR / f'test{n}.mkv'
    hash_file = _HASHES_DIR / f'test{n}.sha256'

    if not path.exists():
        _download(n, path)
        hash_new = _sha256(path)
        if hash_file.exists():
            saved = hash_file.read_text(encoding='ascii').strip()
            if hash_new != saved:
                path.unlink(missing_ok=True)
                pytest.fail(
                    f'test{n}.mkv hash mismatch after download — '
                    f'official file may have been updated. '
                    f'Delete {hash_file} to accept the new version.\n'
                    f'Cached:     {saved}\n'
                    f'Downloaded: {hash_new}'
                )
        hash_file.write_text(f'{hash_new}\n', encoding='ascii')
        return path

    if not hash_file.exists():
        hash_file.write_text(f'{_sha256(path)}\n', encoding='ascii')
        return path

    current = _sha256(path)
    saved = hash_file.read_text(encoding='ascii').strip()
    if current == saved:
        return path

    path.unlink(missing_ok=True)
    _download(n, path)
    hash_new = _sha256(path)
    if hash_new == saved:
        return path

    path.unlink(missing_ok=True)
    pytest.fail(
        f'test{n}.mkv hash mismatch persists after re-download.\n'
        f'Cached:     {saved}\n'
        f'Downloaded: {hash_new}\n'
        f'Official file may have been updated. '
        f'Delete {hash_file} to accept the new version.'
    )


@pytest.fixture(scope='session')
def test1_mkv_path() -> Path:
    return _ensure_test_file(1)


@pytest.fixture(scope='session')
def test2_mkv_path() -> Path:
    return _ensure_test_file(2)


@pytest.fixture(scope='session')
def test3_mkv_path() -> Path:
    return _ensure_test_file(3)


@pytest.fixture(scope='session')
def test4_mkv_path() -> Path:
    return _ensure_test_file(4)


@pytest.fixture(scope='session')
def test5_mkv_path() -> Path:
    return _ensure_test_file(5)


@pytest.fixture(scope='session')
def test6_mkv_path() -> Path:
    return _ensure_test_file(6)


@pytest.fixture(scope='session')
def test7_mkv_path() -> Path:
    return _ensure_test_file(7)


@pytest.fixture(scope='session')
def test8_mkv_path() -> Path:
    return _ensure_test_file(8)
