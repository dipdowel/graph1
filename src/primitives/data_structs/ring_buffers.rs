//! This module provides two types of FIFO ring buffers:
//!
//! - `RingBuffer<T, const N: usize>`: A fixed-capacity, stack-allocated ring buffer (more performant).
//! - `DynamicRingBuffer<T>`: A heap-allocated, resizable ring buffer (somewhat slower due to heap allocation, but resizable in runtime).
//!
//! Both types are FIFO (first-in, first-out), and when full, automatically discard the oldest element.

//
//
//
//


/// A fixed-size, stack-allocated ring buffer that overwrites the oldest values when full.
///
/// This is ideal for real-time contexts where heap allocation is not necessary or desirable.
/// Uses a `[T; N]` array, so capacity is known at compile time.
///
/// # Type Parameters
/// - `T`: The type of values stored in the buffer. Must implement `Copy` and `Default`.
/// - `N`: The compile-time constant capacity of the buffer.
#[derive(Debug)]
pub struct RingBuffer<T: Copy, const N: usize> {
    buffer: [T; N],
    head: usize,
    len: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    /// Creates a new, empty ring buffer.
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); N],
            head: 0,
            len: 0,
        }
    }

    /// Pushes a new value into the buffer.
    /// If the buffer is full, the oldest value is overwritten.
    pub fn push(&mut self, value: T) {
        let index = (self.head + self.len) % N;
        self.buffer[index] = value;

        if self.len < N {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % N; // Discard oldest
        }
    }

    /// Gets the number of elements currently in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Returns an element by FIFO index (0 = oldest).
    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            let idx = (self.head + index) % N;
            Some(self.buffer[idx])
        } else {
            None
        }
    }


    /// Retrieves the first (the oldest) element in the buffer, if any.
    pub fn first(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self.buffer[self.head])
        } else {
            None
        }
    }

    /// Retrieves the last (the newest) element in the buffer, if any.
    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            let idx = (self.head + self.len - 1) % N;
            Some(&self.buffer[idx])
        } else {
            None
        }
    }


    /// Clears the buffer without changing its capacity.
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

/// A dynamically allocated ring buffer. Similar to `RingBuffer`, but resizable at runtime.
#[derive(Debug)]
pub struct DynamicRingBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize,
    len: usize,
}

impl<T: Clone + Default> DynamicRingBuffer<T> {
    /// Creates a new ring buffer with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity must be greater than zero");
        }
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize(capacity, T::default());

        Self {
            buffer,
            capacity,
            head: 0,
            len: 0,
        }
    }

    /// Pushes a value into the buffer, overwriting the oldest if full.
    pub fn push(&mut self, value: T) {
        //
        let index = (self.head + self.len) % self.capacity;
        self.buffer[index] = value;

        if self.len < self.capacity {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
    }

    /// Gets the number of stored elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    /// Retrieves an element by logical index (0 = oldest).
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            let idx = (self.head + index) % self.capacity;
            self.buffer.get(idx)
        } else {
            None
        }
    }

    /// Retrieves the first (the oldest) element in the buffer, if any.
    pub fn first(&self) -> Option<&T> {
        if self.len > 0 {
            self.buffer.get(self.head)
        } else {
            None
        }
    }

    /// Retrieves the last (the newest) element in the buffer, if any.
    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            let idx = (self.head + self.len - 1) % self.capacity;
            self.buffer.get(idx)
        } else {
            None
        }
    }



    /// Resizes the buffer, discarding oldest values if shrinking.
    pub fn resize(&mut self, new_capacity: usize) {
        let mut new_buffer = vec![T::default(); new_capacity];
        let items_to_copy = self.len.min(new_capacity);

        for i in 0..items_to_copy {
            new_buffer[i] = self.get(i).unwrap().clone();
        }

        self.buffer = new_buffer;
        self.capacity = new_capacity;
        self.head = 0;
        self.len = items_to_copy;
    }

    /// Clears all contents without changing capacity.
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_push_and_get() {
        let mut buf = RingBuffer::<i32, 3>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert_eq!(buf.get(0), Some(1));
        assert_eq!(buf.get(1), Some(2));
        assert_eq!(buf.get(2), Some(3));

        buf.push(4); // Overwrites 1
        assert_eq!(buf.get(0), Some(2));
        assert_eq!(buf.get(1), Some(3));
        assert_eq!(buf.get(2), Some(4));
    }

    #[test]
    fn dynamic_buffer_push_resize() {
        let mut buf = DynamicRingBuffer::<i32>::with_capacity(2);
        buf.push(10);
        buf.push(20);
        buf.push(30);
        assert_eq!(buf.get(0), Some(&20));
        assert_eq!(buf.get(1), Some(&30));

        buf.resize(4);
        buf.push(40);
        buf.push(50);
        assert_eq!(buf.len(), 4);
        assert_eq!(buf.get(0), Some(&20));
        assert_eq!(buf.get(3), Some(&50));
    }

    #[test]
    fn ring_buffer_clear() {
        let mut buf = RingBuffer::<i32, 5>::new();
        buf.push(1);
        buf.push(2);
        buf.clear();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.get(0), None);
    }

    #[test]
    fn dynamic_buffer_clear() {
        let mut buf = DynamicRingBuffer::<i32>::with_capacity(5);
        buf.push(1);
        buf.push(2);
        buf.clear();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.get(0), None);
    }

    #[test]
    fn ring_buffer_first_last() {
        let mut buf = RingBuffer::<i32, 3>::new();
        assert_eq!(buf.first(), None);
        assert_eq!(buf.last(), None);
        buf.push(10);
        assert_eq!(buf.first(), Some(&10));
        assert_eq!(buf.last(), Some(&10));
        buf.push(20);
        assert_eq!(buf.first(), Some(&10));
        assert_eq!(buf.last(), Some(&20));
        buf.push(30);
        buf.push(40); // Overwrites 10
        assert_eq!(buf.first(), Some(&20));
        assert_eq!(buf.last(), Some(&40));
    }

    #[test]
    fn dynamic_buffer_first_last() {
        let mut buf = DynamicRingBuffer::<i32>::with_capacity(3);
        assert_eq!(buf.first(), None);
        assert_eq!(buf.last(), None);
        buf.push(100);
        assert_eq!(buf.first(), Some(&100));
        assert_eq!(buf.last(), Some(&100));
        buf.push(200);
        assert_eq!(buf.first(), Some(&100));
        assert_eq!(buf.last(), Some(&200));
        buf.push(300);
        buf.push(400); // Overwrites 100
        assert_eq!(buf.first(), Some(&200));
        assert_eq!(buf.last(), Some(&400));
    }

    #[test]
    #[should_panic(expected = "Capacity must be greater than zero")]
    fn dynamic_buffer_zero_capacity() {
        let _ = DynamicRingBuffer::<i32>::with_capacity(0);
    }

}
