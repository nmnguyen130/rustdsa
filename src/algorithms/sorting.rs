//! IntroSort reference implementation (single-pivot)
//! Based on "IntroSort: A Comprehensive Guide to Hybrid Sorting"
//! - QuickSort with depth monitoring
//! - Switch to HeapSort at depth limit 2*floor(log2(n))
//! - Use Insertion Sort for partitions with n <= 16

const INSERTION_SORT_THRESHOLD: usize = 16;

/// Public API: Sort std
pub fn sort_std(arr: &mut [i64]) {
    arr.sort();
}

/// Public API: IntroSort for i64 slices (in-place)
pub fn introsort(arr: &mut [i64]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    let max_depth = 2 * ((n as f64).log2().floor() as usize);
    introsort_inner(arr, 0, n - 1, max_depth);
}

fn introsort_inner(arr: &mut [i64], lo: usize, hi: usize, depth_limit: usize) {
    if lo >= hi {
        return;
    }
    let len = hi - lo + 1;

    if len <= INSERTION_SORT_THRESHOLD {
        insertion_sort(arr, lo, hi);
        return;
    }

    if depth_limit == 0 {
        heapsort(arr, lo, hi);
        return;
    }

    // Single-pivot quicksort with median-of-three pivot selection
    let p = partition(arr, lo, hi);
    if p > 0 {
        // guard against underflow
        introsort_inner(arr, lo, p - 1, depth_limit - 1);
    }
    introsort_inner(arr, p + 1, hi, depth_limit - 1);
}

#[inline]
fn median_of_three(arr: &mut [i64], a: usize, b: usize, c: usize) -> usize {
    // Partially order a, b, c and return index of median; swap to improve locality
    if arr[a] > arr[b] {
        arr.swap(a, b);
    }
    if arr[b] > arr[c] {
        arr.swap(b, c);
    }
    if arr[a] > arr[b] {
        arr.swap(a, b);
    }
    b
}

#[inline]
fn partition(arr: &mut [i64], lo: usize, hi: usize) -> usize {
    let mid = lo + ((hi - lo) >> 1);
    let m = median_of_three(arr, lo, mid, hi);
    arr.swap(m, hi); // use arr[hi] as pivot
    let pivot = arr[hi];

    // Lomuto partition (simple and branch-light)
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}

#[inline]
fn insertion_sort(arr: &mut [i64], lo: usize, hi: usize) {
    for i in lo + 1..=hi {
        let key = arr[i];
        let mut j = i;
        while j > lo && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

// Heapsort fallback
fn heapsort(arr: &mut [i64], lo: usize, hi: usize) {
    let n = hi - lo + 1;
    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, lo + i, lo, hi);
    }
    // Extract elements one by one
    for end in (lo..=hi).rev() {
        arr.swap(lo, end);
        if end == lo {
            break;
        }
        heapify(arr, lo, lo, end - 1);
    }
}

#[inline]
fn heapify(arr: &mut [i64], root: usize, base: usize, last: usize) {
    let mut largest = root;
    loop {
        let left = base + 2 * (largest - base) + 1;
        let right = left + 1;
        let mut max_idx = largest;
        if left <= last && arr[left] > arr[max_idx] {
            max_idx = left;
        }
        if right <= last && arr[right] > arr[max_idx] {
            max_idx = right;
        }
        if max_idx == largest {
            break;
        }
        arr.swap(largest, max_idx);
        largest = max_idx;
    }
}
