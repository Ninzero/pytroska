"""Tests for the Pytroska exception hierarchy."""

from pytroska import ParseError, PytroskaError, CorruptedError, UnsupportedError


def test_pytroska_error_is_exception() -> None:
    assert issubclass(PytroskaError, Exception)


def test_parse_error_inheritance() -> None:
    assert issubclass(ParseError, PytroskaError)


def test_corrupted_error_inheritance() -> None:
    assert issubclass(CorruptedError, PytroskaError)


def test_unsupported_error_inheritance() -> None:
    assert issubclass(UnsupportedError, PytroskaError)
