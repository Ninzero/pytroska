"""Smoke tests for pytroska."""

import pytroska
from pytroska import core_version


def test_import() -> None:
    assert hasattr(pytroska, '__version__')


def test_core_version_returns_string() -> None:
    version = core_version()
    assert isinstance(version, str)
    assert len(version) > 0


def test_core_version_format() -> None:
    version = core_version()
    parts = version.split('.')
    assert len(parts) == 3


def test_package_version() -> None:
    assert pytroska.__version__ == '0.1.0'
