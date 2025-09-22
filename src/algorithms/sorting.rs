// Pure Rust sorting algorithms

/// Baseline using Rust's stable sort (for benchmarking)
pub fn quicksort_std(mut arr: Vec<i64>) -> Vec<i64> {
    arr.sort();
    arr
}

/// Custom introsort-based quicksort (dual-pivot + insertion sort small, heapsort fallback)
pub fn quicksort(mut arr: Vec<i64>) -> Vec<i64> {
    let n = arr.len();
    if n <= 1 {
        return arr;
    }
    let max_depth = 2 * ((n as f64).log2().floor() as usize);
    introsort(&mut arr, 0, n - 1, max_depth);
    arr
}

fn introsort(arr: &mut [i64], low: usize, high: usize, max_depth: usize) {
    if low >= high {
        return;
    }
    if high - low <= 32 {
        insertion_sort(arr, low, high);
        return;
    }
    if max_depth == 0 {
        heapsort(arr, low, high);
        return;
    }
    dual_pivot_quicksort(arr, low, high, max_depth - 1);
}

fn dual_pivot_quicksort(arr: &mut [i64], low: usize, high: usize, max_depth: usize) {
    // Choose 2 pivots (by index). Ensure pivot1 <= pivot2 by swapping indices, not array values.
    let (mut p1_idx, mut p2_idx) = choose_dual_pivots(arr, low, high);
    if arr[p1_idx] > arr[p2_idx] {
        core::mem::swap(&mut p1_idx, &mut p2_idx);
    }
    if p1_idx != low {
        arr.swap(low, p1_idx);
    }
    // If pivot2 was originally at `low`, it is now at `p1_idx` after the first swap.
    if p2_idx == low {
        p2_idx = p1_idx;
    }
    if p2_idx != high {
        arr.swap(high, p2_idx);
    }

    let pivot1 = arr[low];
    let pivot2 = arr[high];

    let (lt, gt) = dual_pivot_partition(arr, low + 1, high - 1, pivot1, pivot2);

    // Set pivot to the correct position
    if lt > low {
        arr.swap(low, lt - 1);
    }
    if gt + 1 <= high {
        arr.swap(high, gt + 1);
    }

    // Recursively sort the three partitions
    // Left partition: [low .. lt - 2] (elements < pivot1)
    if lt > low + 1 {
        introsort(arr, low, lt - 2, max_depth);
    }
    if lt <= gt {
        introsort(arr, lt, gt, max_depth);
    }
    if gt + 1 < high {
        introsort(arr, gt + 2, high, max_depth);
    }
}

fn dual_pivot_partition(
    arr: &mut [i64],
    left: usize,
    right: usize,
    pivot1: i64,
    pivot2: i64
) -> (usize, usize) {
    let mut i = left;
    let mut lt = left;
    let mut gt = right;

    while i <= gt {
        if arr[i] < pivot1 {
            arr.swap(i, lt);
            lt += 1;
            i += 1;
        } else if arr[i] > pivot2 {
            arr.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }
    (lt, gt)
}

fn choose_dual_pivots(arr: &[i64], low: usize, high: usize) -> (usize, usize) {
    let mid = low + (high - low) / 2;
    let candidates = [
        (low, arr[low]),
        (mid, arr[mid]),
        (high, arr[high]),
        (low + (mid - low) / 2, arr[low + (mid - low) / 2]),
        (mid + (high - mid) / 2, arr[mid + (high - mid) / 2]),
    ];

    let mut sorted_indices: Vec<usize> = (0..candidates.len()).collect();
    sorted_indices.sort_by_key(|&i| candidates[i].1);

    // Return the 2nd and 4th smallest elements
    (candidates[sorted_indices[1]].0, candidates[sorted_indices[3]].0)
}

fn insertion_sort(arr: &mut [i64], low: usize, high: usize) {
    for i in low + 1..=high {
        let key = arr[i];
        let mut j = i;
        while j > low && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn heapsort(arr: &mut [i64], low: usize, high: usize) {
    let count = high - low + 1;

    // Build max heap
    for start in (0..count / 2).rev() {
        heapify(arr, start + low, low, high);
    }

    // Extract elements
    for end in (low..=high).rev() {
        arr.swap(low, end);
        if end == low {
            break;
        }
        heapify(arr, low, low, end - 1);
    }
}

fn heapify(arr: &mut [i64], root: usize, low: usize, high: usize) {
    let mut largest = root;
    let left = 2 * (root - low) + 1 + low;
    let right = left + 1;

    if left <= high && arr[left] > arr[largest] {
        largest = left;
    }
    if right <= high && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != root {
        arr.swap(root, largest);
        heapify(arr, largest, low, high);
    }
}
