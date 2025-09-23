import random
import time

from rustdsa import sorting


def is_sorted(a: list[int]) -> bool:
    return all(a[i] <= a[i + 1] for i in range(len(a) - 1))

def benchmark_sort(seed: int = 42) -> None:
    n = 10_000_000
    rng = random.Random(seed)
    data = [rng.randint(-10**9, 10**9) for _ in range(n)]

    data_py = data[:]
    data_rs_custom = data[:]
    data_rs_std = data[:]

    # Python Timsort
    t0 = time.perf_counter()
    data_py.sort()
    t1 = time.perf_counter()

    # Rust custom quicksort (introsort-based)
    t2 = time.perf_counter()
    rs_custom_sorted = sorting.sort(data_rs_custom)
    t3 = time.perf_counter()

    # Rust std sort baseline
    t4 = time.perf_counter()
    rs_std_sorted = sorting.sort_std(data_rs_std)
    t5 = time.perf_counter()

    # Validate correctness
    assert rs_custom_sorted == data_py
    assert rs_std_sorted == data_py

    py_time = t1 - t0
    rs_custom_time = t3 - t2
    rs_std_time = t5 - t4

    speedup_custom = py_time / rs_custom_time if rs_custom_time > 0 else float("inf")
    speedup_std = py_time / rs_std_time if rs_std_time > 0 else float("inf")

    print(f"Python sort: {py_time:.3f}s")
    print(f"Rust quicksort (custom): {rs_custom_time:.3f}s")
    print(f"Rust sort (std): {rs_std_time:.3f}s")
    print(f"Speedup custom vs Python: {speedup_custom:.2f}x")
    print(f"Speedup std vs Python: {speedup_std:.2f}x")


if __name__ == "__main__":
    benchmark_sort()
