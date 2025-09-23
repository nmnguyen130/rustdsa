# IntroSort: Hướng dẫn đầy đủ (kèm thuật toán và giả mã)

IntroSort là thuật toán sắp xếp lai (hybrid) kết hợp ưu điểm của QuickSort (trung bình rất nhanh), HeapSort (bảo đảm O(n log n) trong trường hợp xấu), và Insertion Sort (tối ưu cho mảng nhỏ/gần sắp xếp). Ý tưởng chính:

- Bắt đầu bằng QuickSort.
- Theo dõi độ sâu đệ quy. Nếu vượt quá ngưỡng 2\*floor(log2(n)), chuyển sang HeapSort để tránh trường hợp xấu.
- Khi kích thước đoạn con nhỏ hơn một ngưỡng (thường là 16), dùng Insertion Sort để tiết kiệm chi phí đệ quy và nhánh.

Tài liệu tham khảo ngắn: [“IntroSort: A Comprehensive Guide to Hybrid Sorting – The Research Scientist Pod”](https://researchdatapod.com/introsort/) và Wikipedia: [Introsort](https://en.wikipedia.org/wiki/Introsort).

---

## Cấu phần thuật toán

- QuickSort (đơn pivot) hoặc dual-pivot QuickSort.
- Chiến lược chọn pivot (median-of-three, median-of-five, hoặc 5-sample).
- Partition scheme (Lomuto, Hoare, 3-way nếu nhiều phần tử trùng).
- Heapsort làm đường lùi (fallback) khi vượt độ sâu tối đa.
- Insertion Sort cho phân đoạn nhỏ (ví dụ n ≤ 16).

---

## Pseudocode tổng thể

Ngưỡng dùng thường gặp:

- `depth_limit = 2 * floor(log2(n))`
- `INSERTION_SORT_THRESHOLD = 16`

```text
procedure introsort(A: array)
    if length(A) ≤ 1 then
        return
    maxdepth = 2 * floor(log2(length(A)))
    introsort_helper(A, 0, length(A)-1, maxdepth)

procedure introsort_helper(A: array, low: int, high: int, depth_limit: int)
    n = high - low + 1
    if n ≤ INSERTION_SORT_THRESHOLD then
        insertion_sort(A, low, high)
        return
    if depth_limit = 0 then
        heapsort(A, low, high)
        return

    // Chọn pivot (ví dụ median-of-three)
    p = partition(A, low, high) // QuickSort partition
    if p > low then
        introsort_helper(A, low, p-1, depth_limit - 1)
    introsort_helper(A, p+1, high, depth_limit - 1)

---

## Chọn pivot (median-of-three)

```text
procedure median_of_three(A: array, i, j, k) -> index
    // Sắp bộ ba để chọn trung vị
    if A[i] > A[j] then swap(A[i], A[j])
    if A[j] > A[k] then swap(A[j], A[k])
    if A[i] > A[j] then swap(A[i], A[j])
    return j // A[j] bây giờ xấp xỉ trung vị
```

- Ưu điểm: chi phí thấp, giảm rủi ro pivot tệ.
- Ngoài ra có thể dùng median-of-five hoặc 5-sample không cấp phát (sử dụng mạng sắp xếp nhỏ) nếu muốn ổn định hơn khi dữ liệu có cấu trúc.

---

## Partition (Lomuto) – đơn giản, dễ đọc

```text
procedure partition(A: array, low, high) -> int
    m = median_of_three(A, low, (low+high)//2, high)
    swap(A[m], A[high])
    pivot = A[high]
    i = low
    for j from low to high-1 do
        if A[j] ≤ pivot then
            swap(A[i], A[j])
            i = i + 1
    swap(A[i], A[high])
    return i
```

- Điểm mạnh: đơn giản, số nhánh ít.
- Điểm yếu: thường hoán vị nhiều hơn so với Hoare.

## Partition (Hoare) – ít hoán vị hơn

```text
procedure partition_hoare(A: array, low, high) -> int
    m = median_of_three(A, low, (low+high)//2, high)
    pivot = A[m]
    i = low - 1
    j = high + 1
    loop
        repeat i = i + 1 until A[i] ≥ pivot
        repeat j = j - 1 until A[j] ≤ pivot
        if i ≥ j then
            return j
        swap(A[i], A[j])
```

- Hoare trả về chỉ số chia khác (j), cần chú ý khi đệ quy.

## Partition 3-way (nhiều phần tử trùng)

```text
procedure partition_3way(A: array, low, high) -> (lt, gt)
    pivot = A[(low+high)//2]
    i = low
    lt = low
    gt = high
    while i ≤ gt do
        if A[i] < pivot then
            swap(A[i], A[lt]); i = i + 1; lt = lt + 1
        else if A[i] > pivot then
            swap(A[i], A[gt]); gt = gt - 1
        else
            i = i + 1
    return (lt, gt)
```

- Khi dùng 3-way, đệ quy vào hai vùng: `[lo..lt-1]` và `[gt+1..hi]`.

---

## Insertion Sort (cho phân đoạn nhỏ)

```text
procedure insertion_sort(A: array, lo, hi)
    for i from lo+1 to hi do
        key = A[i]
        j = i
        while j > lo and A[j-1] > key do
            A[j] = A[j-1]
            j = j - 1
        A[j] = key
```

- Có thể tối ưu bằng binary insertion (giảm so sánh) và dịch khối bằng memmove/copy để tăng tốc.

---

## HeapSort (đường lùi khi quá sâu)

```text
procedure heapsort(A: array, lo, hi)
    n = hi - lo + 1
    // Xây max-heap
    for s from floor(n/2)-1 down to 0 do
        heapify(A, lo + s, lo, hi)

    // Trích phần tử lớn nhất về cuối
    for end from hi down to lo do
        swap(A[lo], A[end])
        if end == lo then break
        heapify(A, lo, lo, end-1)

procedure heapify(A, root, base, last)
    largest = root
    loop
        left = base + 2*(largest-base) + 1
        right = left + 1
        max_idx = largest
        if left ≤ last and A[left] > A[max_idx] then max_idx = left
        if right ≤ last and A[right] > A[max_idx] then max_idx = right
        if max_idx == largest then break
        swap(A[largest], A[max_idx])
        largest = max_idx
```

---

## Lựa chọn tham số và tinh chỉnh

- `INSERTION_SORT_THRESHOLD`: 16 là giá trị hay dùng; có thể thử 24–32 phụ thuộc kiến trúc CPU và đặc điểm dữ liệu.
- `depth_limit`: chuẩn 2\*floor(log2(n)); có thể tinh chỉnh nhưng giá trị này an toàn và phổ biến.
- Chọn partition scheme phù hợp:
  - Dữ liệu nhiều trùng: 3-way partition giúp giảm độ sâu và chi phí.
  - Bình thường: Lomuto dễ viết/đọc; Hoare có thể ít swap hơn.
- Pivot selection:
  - `median-of-three`: rẻ, phổ biến.
  - `median-of-five`/`5-sample` (không cấp phát): ổn định hơn trên dữ liệu có cấu trúc, nhưng chi phí cao hơn một chút.

---

## Độ phức tạp

- Thời gian:
  - Trung bình: O(n log n)
  - Tệ nhất: O(n log n) nhờ fallback HeapSort
- Bộ nhớ: O(log n) do độ sâu đệ quy (có thể giảm thêm bằng tail-recursion optimization/loop hóa nhánh lớn).
- Không ổn định (giống QuickSort/HeapSort chuẩn).

---

## Lưu đồ (tóm tắt)

```mermaid
flowchart TD
    A[Start Introsort] --> B{n <= threshold?}
    B -- Yes --> C[Insertion Sort]
    B -- No --> D{depth_limit == 0?}
    D -- Yes --> E[HeapSort]
    D -- No --> F[Choose pivot]
    F --> G[Partition]
    G --> H[introsort_helper(left, depth-1)]
    G --> I[introsort_helper(right, depth-1)]
    C --> J[Return]
    E --> J
    H --> J
    I --> J
```

---

## Gợi ý sử dụng trong project này

- `algorithms/sorting.rs` hiện triển khai phiên bản IntroSort đơn-pivot theo hướng dẫn, dùng median-of-three + Lomuto, `INSERTION_SORT_THRESHOLD = 16`, `depth_limit = 2*floor(log2(n))`.
