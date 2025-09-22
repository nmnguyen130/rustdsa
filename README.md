# rustdsa

High-performance algorithms library implemented in Rust with Python bindings using pyo3 + maturin. Designed to accelerate heavy computations by offloading to Rust while exposing a clean Python API.

## Features

- Sorting (`rustdsa.sorting`)
  - Class: `Sorting()` with methods:
    - `quicksort(values: list[int]) -> list[int]`
    - `mergesort(values: list[int]) -> list[int]`
    - `is_sorted(values: list[int]) -> bool`
- Searching (`rustdsa.searching`)
  - Class: `Searching()` with methods:
    - `linear_search(values: list[int], target: int) -> int | None`
    - `binary_search(values_sorted: list[int], target: int) -> int | None`

## Requirements

- Rust toolchain (install via https://rustup.rs)
- Python 3.8–3.13
- maturin (Python packaging/build tool for Rust extensions)

Install maturin (recommended via pipx):

```powershell
# Windows PowerShell
pipx install maturin
# or
python -m pip install --upgrade maturin
```

## Development (editable install)

Build and install the extension into your current Python environment for development:

```powershell
# From the project root (d:\Coding\Lib\rustdsa)
maturin develop --release
```

Then, in Python:

```python
import rustdsa

# Sorting (class-based API)
s = rustdsa.sorting.Sorting()
print(s.quicksort([3, 1, 2]))       # [1, 2, 3]
print(s.mergesort([5, 4, 6, 1]))    # [1, 4, 5, 6]
print(s.is_sorted([1, 2, 2, 3]))    # True

# Searching
arr = [1, 3, 5, 7, 9]
f = rustdsa.searching.Searching()
print(f.linear_search(arr, 7))    # 3
print(f.binary_search(arr, 5))    # 2
```

Run tests (pytest is optional but recommended):

```powershell
python -m pip install -U pytest
pytest -q
```

## Building Wheels

Produce wheels under `dist/` for distribution:

```powershell
maturin build --release
```

To install the wheel locally:

```powershell
python -m pip install --force-reinstall --no-index --find-links dist rustdsa
```

## Publishing to PyPI

Make sure you have an account and API token.

```powershell
# TestPyPI
maturin publish --repository testpypi --skip-existing

# PyPI
maturin publish --skip-existing
```

If you need to pass a token explicitly:

```powershell
$env:MATURIN_PYPI_TOKEN = "pypi-xxxxxxxxxxxxxxxx"
maturin publish --skip-existing
```

## Project Layout

- `Cargo.toml`: Rust crate configuration (cdylib + pyo3)
- `src/lib.rs`: Exposes Python submodules `sorting` and `searching` via `#[pymodule]` (PyO3 0.26 `Bound<PyModule>`)
- `src/algorithms/`: Core algorithms implemented in pure Rust
  - `mod.rs`: module declarations for `sorting` and `searching`
  - `sorting.rs`: quicksort, mergesort, helpers
  - `searching.rs`: linear and binary search
- `pyproject.toml`: Python package metadata and build backend (maturin)
- `tests/`: Python tests
- `.gitignore`: ignores Rust/Python build artifacts
