from pathlib import Path
import os
import numpy as np
import pytest
import subprocess


def build_module():
    env = os.environ.copy()
    env["PATH"] = f"{Path.home()}/.cargo/bin:{env.get('PATH', '')}"
    subprocess.run(["maturin", "develop"], check=True, env=env)


@pytest.fixture(scope="module")
def build_fixture():
    build_module()


@pytest.fixture(scope="module")
def test_inputs():
    npz_path = Path("tests/test_inputs.npz")
    if npz_path.exists():
        return np.load(npz_path)["data"]
    return np.random.randn(100, 4)


def test_annotate_scanlines_stub(build_fixture, test_inputs):
    from scanline_annotator import annotate_scanlines

    output = annotate_scanlines(test_inputs)
    assert isinstance(output, np.ndarray)
    assert output.shape == test_inputs.shape
