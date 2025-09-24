# Hướng dẫn cấu trúc dữ liệu lõi (`src/core/datastructures/`)

Tài liệu này mô tả ba mô-đun trung tâm bên trong thư mục `src/core/datastructures/`. Mỗi mô-đun đều được thiết kế để trở thành nền tảng hiệu năng cao cho các thuật toán phía Rust, sau đó được binding sang Python thông qua PyO3.

---

## 1. `array.rs`

**Mục tiêu**

- Chuẩn hóa giao diện cho mọi cấu trúc "giống mảng" bằng trait `ArrayLike<T>`.
- Hỗ trợ streaming/batch (e.g., `ChunkIter`, `RollingMean`) với hiệu suất cao hơn NumPy.
- Tăng tốc qua SIMD, multi-thread, và zero-copy từ Python (PyO3).

**Thành phần chính**

- `ArrayLike<T>`
  - `len(&self)`, `is_empty(&self)`
  - `as_slice(&self)` -> `&[T]`
  - `as_aligned_slice(&self)` -> `Option<&[T]>` (cho SIMD)
  - `chunk_iter(&self, chunk_size)` chia dữ liệu thành từng khối.
  - `par_chunk_iter(&self, chunk_size)` (Rayon)
- `ChunkIter<'a, T>`: iterator không cấp phát, duyệt slice theo batch.
- `RollingMean`
  - `new(window: usize)`
  - `push(value: f64) -> f64`
  - `push_batch<A: ArrayLike<f64>>(&mut self, batch: &A)` (có SIMD nếu slice align)
  - `reset()` và `current_mean()`
- `rolling_mean<A: ArrayLike<f64>>(data: &A, window: usize)` – xử lý toàn mảng.
- `PyArrayLike`: wrapper zero-copy cho NumPy array qua PyBuffer.

**Ghi chú triển khai**

- `RollingMean`
  - Dùng buffer tròn (`Vec<f64>`) và tổng cộng dồn -> update trung bình O(1).
  - `push_batch` dùng SIMD (`std::simd`) nếu dữ liệu aligned.
  - Hỗ trợ compile-time window (`RollingMean<const N: usize>`) để tối ưu alloc (stack-based).
- Trait `ArrayLike<T>` có impl mặc định cho `[T]`, `Vec<T>`, `ndarray::ArrayView1<T>`, `PyArrayLike<T>`.
- `par_chunk_iter` dùng Rayon cho xử lý song song batch – hữu ích với data lớn.
- `PyArrayLike<T>`: dùng `PyBuffer` để borrow NumPy arrays không copy.
- Benchmark dùng `criterion`, so sánh vs NumPy (scalar + vectorized) để tối ưu.

**Gợi ý nâng cao**

- **Blanket impls**: `impl<T> ArrayLike<T> for &[T]`, `[T; N]`, giúp dùng dễ hơn, không cần wrapper.
- **Compile-time window size**: `RollingMean<const N: usize>` -> dùng stack, không heap alloc.
- **WASM/portable**: nếu cần chạy inference trong browser, edge, hoặc embedded.
- **FFI-safe API**: cho C/C++ gọi lại lib qua `extern "C"` wrapper hoặc C header gen (e.g., cbindgen).
- **Fallback scalar path**: nếu slice không align SIMD, tự động fallback sang scalar an toàn.

---

## 2. `vector.rs`

**Mục tiêu**

- Định nghĩa `SharedVector<T>` dựa trên `Arc<Vec<T>>` để chia sẻ dữ liệu lớn giữa nhiều luồng hoặc giữa Python ↔ Rust mà không copy.

**Thành phần chính**

- `SharedVector<T>`
  - `from_vec(vec: Vec<T>)`
  - `as_arc(&self) -> &Arc<Vec<T>>`
  - `as_slice(&self) -> &[T]`
  - `strong_count(&self) -> usize`
  - `make_mut(&mut self) -> &mut Vec<T>` (copy-on-write khi còn tham chiếu khác)
- Trait
  - `impl ArrayLike<T> for SharedVector<T>`
  - `impl Deref<Target = [T]>`
  - `impl DerefMut` (trả `&mut [T]` thông qua `Arc::make_mut(...).as_mut_slice()`)
- Hàm `collect_shared(iter)` thu thập iterator kích thước xác định thành `SharedVector<T>`.

**Ghi chú triển khai**

- API giữ vùng nhớ liên tục nên rất thân thiện với SIMD, rayon hoặc FFI khác.
- Khi binding sang Python, dữ liệu vẫn lưu trong cùng buffer do `Arc`, tránh copy kép.

---

## 3. `linked_list.rs`

**Mục tiêu**

- `AppendOnlyList<T>` phục vụ tác vụ ingest/streaming: chỉ append, đọc tuần tự, snapshot thành `Vec<T>` khi cần.

**Thành phần chính**

- `AppendOnlyList<T>`
  - `new()`, `len()`, `is_empty()`
  - `push(&mut self, value: T)`
  - `iter(&self) -> ListIter<T>` trả về iterator clone giá trị (`T: Clone`)
  - `to_vec(&self) -> Vec<T>`
- `ListIter<T>`
  - Duyệt cấu trúc dựa trên `Arc<Node<T>>`
  - Mỗi node: `value` và `Mutex<Option<Arc<Node<T>>>>` cho con trỏ kế tiếp.

**Ghi chú triển khai**

- `Arc` đảm bảo chia sẻ giữa nhiều consumer, `Mutex` đảm bảo cập nhật tail an toàn.
- Iterator chỉ giữ `Arc`, lock từng node khi cần, giúp tránh giữ lock dài.

---

## 4. Liên kết với binding Python

- `bindings/datastructures/array.rs` → lớp `PyRollingMean` + hàm `rolling_mean`.
- `bindings/datastructures/vector.rs` → lớp `PySharedVector` + hàm `collect`.
- `bindings/datastructures/linked_list.rs` → lớp `PyAppendOnlyList` (hỗ trợ `push`, `extend`, `to_list`, ...).

Các binding này gọi trực tiếp API ở `core/datastructures` sau khi chuyển đổi dữ liệu từ Python (`Vec<f64>`, `PyList`, ...).

---

## 5. Định hướng mở rộng

- Hỗ trợ thêm cấu trúc làm việc với `mmap`/IO streaming.
- Bổ sung các cấu trúc xác suất như `CountMinSketch`, `BloomFilter`.
- Đo đạc SIMD/Rayon trên `SharedVector` và `RollingMean`.
- Viết benchmark + test PyO3 để đánh giá throughput và đảm bảo tương thích Python.
