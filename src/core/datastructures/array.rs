#![allow(dead_code)] // Remove later when fully used
#![allow(clippy::missing_docs_in_private_items)]

use std::slice::Chunks;
use rayon::prelude::*;

/// A trait to abtract over array-like data structures that allow
/// efficient, generic access and batch processing.
pub trait ArrayLike<T: Copy + Sync> {
    /// Return the number of elements.
    fn len(&self) -> usize;

    /// Returns true if the array is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the underlying slice.
    fn as_slice(&self) -> &[T];

    /// Returns an aligned slice suitable for SIMD usage if possible.
    ///
    /// Default: returns `None` (assumes unaligned)
    #[inline]
    fn as_aligned_slice(&self) -> Option<&[T]> {
        None
    }

    /// Returns a sequential chunk iterator over the array.
    #[inline]
    fn chunk_iter(&self, chunk_size: usize) -> Chunks<'_, T> {
        self.as_slice().chunks(chunk_size)
    }

    /// Returns a parallel chunk iterator using Rayon.
    ///
    /// Requires `T: Sync`.
    #[inline]
    fn par_chunk_iter(&self, chunk_size: usize) -> rayon::slice::Chunks<'_, T> {
        self.as_slice().par_chunks(chunk_size)
    }
}

/// Blanket implementation for `&[T]`
impl<T: Copy + Sync> ArrayLike<T> for &[T] {
    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]
    fn as_aligned_slice(&self) -> Option<&[T]> {
        // For simplicity, assume alignment is sufficient.
        Some(*self)
    }
}

/// Blanket implementation for `Vec<T>`
impl<T: Copy + Sync> ArrayLike<T> for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }

    #[inline]
    fn as_aligned_slice(&self) -> Option<&[T]> {
        Some(self.as_slice())
    }
}

/// An iterator that yields non-overlapping chunks from a slice.
///
/// Similar to [`slice::chunks()`], but can be customized or extended
/// (e.g., for overlapping, strided, etc.).
pub struct ChunkIter<'a, T> {
    data: &'a [T],
    chunk_size: usize,
    pos: usize,
}

impl<'a, T> ChunkIter<'a, T> {
    /// Creates a new `ChunkIter` over a given slice.
    ///
    /// # Arguments
    /// * `data` - A borrowed slice to iterate over.
    /// * `chunk_size` - Size of each chunk (must be > 0).
    ///
    /// # Panics
    /// Panics if `chunk_size == 0`.
    #[inline]
    pub fn new(data: &'a [T], chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "chunk size must be > 0");
        Self {
            data,
            chunk_size,
            pos: 0,
        }
    }
}

impl<'a, T> Iterator for ChunkIter<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }

        let end = (self.pos + self.chunk_size).min(self.data.len());
        let chunk = &self.data[self.pos..end];
        self.pos = end;
        Some(chunk)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len().saturating_sub(self.pos);
        let chunks = (remaining + self.chunk_size - 1) / self.chunk_size;
        (chunks, Some(chunks))
    }
}

impl<'a, T> ExactSizeIterator for ChunkIter<'a, T> {}

use std::collections::VecDeque;

/// A rolling (moving) mean filter over a fixed-size sliding window.
///
/// Efficient for streaming or batch time-series data.
/// Supports constant-time updates and batch processing.
pub struct RollingMean {
    buffer: VecDeque<f64>,
    window: usize,
    sum: f64,
}

impl RollingMean {
    /// Creates a new `RollingMean` with a given window size.
    ///
    /// # Arguments
    /// * `window` - Size of the rolling window (> 0).
    ///
    /// # Panics
    /// Panics if `window == 0`.
    pub fn new(window: usize) -> Self {
        assert!(window > 0, "window size must be > 0");
        Self {
            buffer: VecDeque::with_capacity(window),
            window,
            sum: 0.0,
        }
    }

    /// Pushes a single value and returns the current rolling mean.
    #[inline]
    #[must_use]
    pub fn push(&mut self, value: f64) -> f64 {
        self.buffer.push_back(value);
        self.sum += value;

        if self.buffer.len() > self.window {
            if let Some(removed) = self.buffer.pop_front() {
                self.sum -= removed;
            }
        }

        self.current_mean()
    }

    /// Pushes a batch of values from any ArrayLike structure.
    pub fn push_batch<A: ArrayLike<f64>>(&mut self, batch: &A) {
        for &v in batch.as_slice() {
            let _ = self.push(v);
        }
    }

    /// Returns the current mean of the buffer.
    #[inline]
    #[must_use]
    pub fn current_mean(&self) -> f64 {
        if self.buffer.is_empty() { 0.0 } else { self.sum / (self.buffer.len() as f64) }
    }

    /// Resets the internal state.
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.sum = 0.0;
    }
}

/// Computes the rolling (moving) mean over the input array.
///
/// Returns a vector of rolling means with the same length as input.
/// Values before the window is filled will use partial means.
///
/// # Arguments
/// * `data` - Input data implementing `ArrayLike<f64>`
/// * `window` - Size of the moving window (> 0)
///
/// # Panics
/// Panics if `window == 0`.
pub fn rolling_mean<A: ArrayLike<f64>>(data: &A, window: usize) -> Vec<f64> {
    assert!(window > 0, "window size must be > 0");

    let mut rm = RollingMean::new(window);
    let mut result = Vec::with_capacity(data.len());

    for &v in data.as_slice() {
        result.push(rm.push(v));
    }

    result
}
