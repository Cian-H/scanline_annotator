import os
from pathlib import Path
import subprocess

import numpy as np
import pytest


def build_module() -> None:
    env = os.environ.copy()
    env["PATH"] = f"{Path.home()}/.cargo/bin:{env.get('PATH', '')}"
    subprocess.run(["maturin", "develop"], check=True, env=env)


@pytest.fixture(scope="module")
def build_fixture() -> None:
    build_module()


@pytest.fixture(scope="module")
def test_inputs():
    npz_path = Path("tests/test_inputs.npz")
    if npz_path.exists():
        return np.load(npz_path, allow_pickle=True)["data"]
    return np.random.randn(100, 4)


def test_annotate_scanlines_float64(build_fixture, test_inputs) -> None:
    from scanline_annotator import annotate_scanlines

    x = test_inputs[:, 0].astype(np.float64)
    y = test_inputs[:, 1].astype(np.float64)

    output = annotate_scanlines(x, y)
    assert isinstance(output, np.ndarray)
    assert output.ndim == 1
    assert output.shape == (x.shape[0],)
    assert output.dtype == np.int32
    assert (output > 0).sum() > 0


def test_annotate_scanlines_float32(build_fixture, test_inputs) -> None:
    from scanline_annotator import annotate_scanlines

    x = test_inputs[:, 0].astype(np.float32)
    y = test_inputs[:, 1].astype(np.float32)

    output = annotate_scanlines(x, y)
    assert isinstance(output, np.ndarray)
    assert output.ndim == 1
    assert output.shape == (x.shape[0],)
    assert output.dtype == np.int32
    assert (output > 0).sum() > 0


def test_annotate_scanlines_mismatched_length(build_fixture) -> None:
    from scanline_annotator import annotate_scanlines

    x = np.array([1.0, 2.0, 3.0], dtype=np.float64)
    y = np.array([1.0, 2.0], dtype=np.float64)

    with pytest.raises(ValueError, match="mismatch"):
        _ = annotate_scanlines(x, y)


def test_annotate_scanlines_invalid_type(build_fixture) -> None:
    from scanline_annotator import annotate_scanlines

    x = np.array([1, 2, 3], dtype=np.int64)
    y = np.array([1, 2, 3], dtype=np.int64)

    with pytest.raises(TypeError):
        _ = annotate_scanlines(x, y)
