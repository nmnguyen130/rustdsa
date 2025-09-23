# rustdsa

High-performance algorithms library implemented in Rust with Python bindings. Designed to accelerate heavy computations by offloading to Rust while exposing a simple Python API.

## Installation

```bash
pip install rustdsa
```

Python 3.8–3.13 is supported.

If a prebuilt wheel is not available for your platform, pip may attempt a source build that requires a Rust toolchain. In that case, install Rust from https://rustup.rs and try again.

## Features

- Sorting (`rustdsa.sorting` submodule)
  - `sorting.sort(values: list[int]) -> list[int]`
    - Custom introsort variant (dual-pivot quicksort + heapsort fallback + insertion sort for small runs)
  - Note: sorts 64-bit integers (`i64`).

## Usage

```python
from rustdsa import sorting

print(sorting.sort([3, 1, 2]))            # [1, 2, 3]
```

## Notes

- Input and output are Python lists of integers; values are moved into Rust as 64-bit signed integers.
- If you need deterministic behavior across platforms, ensure your integers fit within `i64`.
