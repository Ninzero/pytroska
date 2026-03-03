"""Tests for the Pytroska exception hierarchy."""

import pickle

from pytroska import ParseError, PytroskaError, CorruptedError, UnsupportedError


def test_pytroska_error_is_exception() -> None:
    assert issubclass(PytroskaError, Exception)


def test_parse_error_inheritance() -> None:
    assert issubclass(ParseError, PytroskaError)


def test_corrupted_error_inheritance() -> None:
    assert issubclass(CorruptedError, PytroskaError)


def test_unsupported_error_inheritance() -> None:
    assert issubclass(UnsupportedError, PytroskaError)


def test_exception_module_is_fully_qualified() -> None:
    for exc_type in (CorruptedError, ParseError, PytroskaError, UnsupportedError):
        assert exc_type.__module__ == 'pytroska._pytroska_core'


def test_exception_pickle_roundtrip() -> None:
    for exc_type in (CorruptedError, ParseError, PytroskaError, UnsupportedError):
        original = exc_type('test message')
        restored = pickle.loads(pickle.dumps(original))
        assert type(restored) is exc_type
        assert restored.args == ('test message',)


# TODO(Phase 3, M2): Add dedicated tests for `From<PytroskaRustError> for PyErr`
# mapping behavior (including io::Error -> concrete Python OSError subclasses).
