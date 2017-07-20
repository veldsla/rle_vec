//! This crate provides `RleVec`, a vector like structure that stores runs of identical values coded
//! by the value and the number of repeats.
//!
//! If your data consists of long stretches of identical values is can be beneficial to only store
//! the number of times each value occurs. This can result in significant space savings, but there
//! is a cost. Accessing an arbitrary index requires a binary search over the stored runs resulting
//! in a O(log n) complexity versus O(1) for a normal vector. Other complexities are in the table
//! where n is equal to the number of runs, not the length of a comparable Vec.
//!
//! |        |push|index   |set with breaking a run|set without breaking a run|insert with breaking a run|insert without breaking a run|
//! |--------|----|--------|-----------------------|--------------------------|--------------------------|-----------------------------|
//! |`RleVec`|O(1)|O(log&nbsp;n)|O((log&nbsp;n)&nbsp;+&nbsp;2n)|O(log&nbsp;n)|O((log&nbsp;n)&nbsp;+&nbsp;2n)|O((log&nbsp;n)&nbsp;+&nbsp;n)|
//! |`Vec`|O(1)|O(1)|O(1)*| |O(n)| |
//!
//! *\*Benchmarks show that setting `vec[idx] = value` is a lot slower than getting `vec[idx]`*
//!

use std::iter::FromIterator;
use std::iter::once;
use std::cmp;
use std::ops::Index;

/// The `RleVec` struct handles like a normal vector and supports a subset from the `Vec` methods.
///
/// Not all methods implemented on `Vec` are implemented for `RleVec`. All methods returning a slice
/// cannot work for `RleVec`.
///
/// # Examples:
/// ```
/// # use rle_vec::RleVec;
/// let mut rle = RleVec::new();
///
/// rle.push(10);
/// rle.push(10);
/// rle.push(11);
///
/// assert_eq!(rle[1], 10);
/// assert_eq!(rle[2], 11);
///
/// rle.insert(1, 10);
/// assert_eq!(rle.runs_len(), 2);
///
/// rle.set(0, 1);
/// assert_eq!(rle.runs_len(), 3);
/// ```
///
/// `RleVec` can be constructed from `Iterators` and be iterated over just like a `Vec`.
///
/// ```
/// # use rle_vec::RleVec;
/// let v = vec![0,0,0,1,1,1,1,2,2,3,4,5,4,4,4];
///
/// let mut rle: RleVec<_> = v.into_iter().collect();
///
/// assert_eq!(rle.len(), 15);
/// assert_eq!(rle.runs_len(), 7);
///
/// assert_eq!(rle.iter().nth(10), Some(&4));
/// ```
///
/// An `RleVec` can be indexed like a regular vector, but not mutated. Use `RleVec::set` to change the
/// value at an index.
///
/// ```
/// # use rle_vec::RleVec;
/// let v = vec![0,0,0,1,1,1,1,2,2,3];
/// let mut rle: RleVec<_> = v.into_iter().collect();
///
/// rle.set(1,2);
/// rle.insert(4,4);
///
/// assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), vec![0,2,0,1,4,1,1,1,2,2,3]);
///
/// ```
/// `RleVec::set` and `RleVec::insert` require `T: Clone`.
///
/// # Indexing
///
/// The `RleVec` type allows to access values by index, because it implements the
/// `Index` trait. An example will be more explicit:
///
/// ```
/// # use rle_vec::RleVec;
/// let v = vec![0, 2, 4, 6];
/// let rle: RleVec<_> = v.into_iter().collect();
///
/// println!("{}", rle[1]); // it will display '2'
/// ```
///
/// However be careful: if you try to access an index which isn't in the `RleVec`,
/// your software will panic! You cannot do this:
///
/// ```ignore
/// # use rle_vec::RleVec;
/// let v = vec![0, 2, 4, 6];
/// let rle: RleVec<_> = v.into_iter().collect();
///
/// println!("{}", v[6]); // it will panic!
/// ```
///
/// In conclusion: always check if the index you want to get really exists
/// before doing it.
///
/// # Capacity and reallocation
///
/// The capacity of an rle_vector is the amount of space allocated for any future
/// elements that will be added onto the rle_vector. This is not to be confused with
/// the *length*, which specifies the number of actual elements.
/// If an rle_vector's length exceeds its capacity, its capacity
/// will automatically be increased, but its elements will have to be
/// reallocated.
///
/// For example, an rle_vector with capacity 10 and length 0 would be an empty vector
/// with space for 10 more elements. Pushing 10 or fewer elements onto the
/// vector will not change its capacity or cause reallocation to occur. However,
/// if the rle_vector's length is increased to 11, it will have to reallocate, which
/// can be slow. For this reason, it is recommended to use `RleVec::with_capacity`
/// whenever possible to specify how big the rle_vector is expected to get.
#[derive(Debug, Clone)]
pub struct RleVec<T> {
    runs: Vec<InternalRun<T>>,
}

/// Represent a run inside the `RleVec`, can be obtained from the [`iter_runs`](struct.RleVec.html#method.iter_runs). A run is a serie of the same value.
///
/// # Example
///
/// ```
/// # use rle_vec::{RleVec, Run};
/// let rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 3]);
///
/// let mut iterator = rle.runs();
/// assert_eq!(iterator.next(), Some(Run{ len: 4, value: &1 }));
/// assert_eq!(iterator.next(), Some(Run{ len: 2, value: &2 }));
/// assert_eq!(iterator.next(), Some(Run{ len: 1, value: &3 }));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Run<T> {
    /// The length of this run.
    pub len: usize,
    /// The value of this run.
    pub value: T,
}

#[derive(Debug, Clone)]
struct InternalRun<T> {
    end: usize,
    value: T,
}

impl<T> RleVec<T> {
    /// Constructs a new empty `RleVec<T>`.
    ///
    /// The rle_vector will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rle_vec::RleVec;
    /// let rle = RleVec::<i32>::new();
    /// ```
    pub fn new() -> RleVec<T> {
        RleVec { runs: Vec::new() }
    }

    /// Constructs a new empty `RleVec<T>` with capacity for the number of runs.
    ///
    /// Choosing this value requires knowledge about the composition of the data that is going to be inserted.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::with_capacity(10);
    ///
    /// // The rle_vector contains no items, even though it has capacity for more
    /// assert_eq!(rle.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for i in 0..10 {
    ///    rle.push(i);
    /// }
    ///
    /// // The rle_vector contains 10 runs and 10 elements too...
    /// assert_eq!(rle.len(), 10);
    /// assert_eq!(rle.runs_len(), 10);
    ///
    /// // ...but this may make the rle_vector reallocate
    /// rle.push(11);
    /// ```
    pub fn with_capacity(capacity: usize) -> RleVec<T> {
        RleVec { runs: Vec::with_capacity(capacity) }
    }

    /// Returns the number of elements in the rle_vector.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    /// rle.push(1);
    /// rle.push(1);
    /// rle.push(2);
    ///
    /// assert_eq!(rle.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        match self.runs.last() {
            Some(run) => run.end + 1,
            None => 0,
        }
    }

    /// Returns `true` if the rle_vector contains no elements.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    /// assert!(rle.is_empty());
    ///
    /// rle.push(1);
    /// assert!(!rle.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    ///
    /// # Examples
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 3]);
    ///
    /// rle.clear();
    /// assert!(rle.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.runs.clear()
    }

    /// Returns the number of runs
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    /// assert_eq!(rle.runs_len(), 0);
    ///
    /// rle.push(1);
    /// rle.push(1);
    /// assert_eq!(rle.runs_len(), 1);
    ///
    /// rle.push(2);
    /// rle.push(3);
    /// assert_eq!(rle.runs_len(), 3);
    /// ```
    pub fn runs_len(&self) -> usize {
        self.runs.len()
    }

    /// Returns the 0-based start coordinates of the runs
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    /// rle.push(1);
    /// rle.push(1);
    /// rle.push(2);
    /// rle.push(2);
    /// rle.push(3);
    ///
    /// let starts = rle.starts();
    /// assert_eq!(starts, vec![0, 2, 4]);
    /// ```
    pub fn starts(&self) -> Vec<usize> {
        if self.is_empty() { return Vec::new() }
        once(0).chain(self.runs.iter().take(self.runs_len() - 1).map(|r| r.end + 1)).collect()
    }

    /// Returns the 0-based end coordinates of the runs
    pub fn ends(&self) -> Vec<usize> {
        self.runs.iter().map(|r| r.end).collect()
    }

    /// Returns an iterator over values. Comparable to a `Vec` iterator.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    /// rle.push(1);
    /// rle.push(1);
    /// rle.push(2);
    /// rle.push(3);
    ///
    /// let mut iterator = rle.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter { rle: self, run_index: 0, index: 0 }
    }

    /// Returns an iterator that can be used to iterate over the runs.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::{RleVec, Run};
    /// let mut rle = RleVec::new();
    /// rle.push(1);
    /// rle.push(1);
    /// rle.push(2);
    /// rle.push(3);
    ///
    /// let mut iterator = rle.runs();
    ///
    /// assert_eq!(iterator.next(), Some(Run{ len: 2, value: &1 }));
    /// assert_eq!(iterator.next(), Some(Run{ len: 1, value: &2 }));
    /// assert_eq!(iterator.next(), Some(Run{ len: 1, value: &3 }));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn runs(&self) -> Runs<T> {
        Runs { rle: self, index: 0, last_end: 0 }
    }

    fn run_index(&self, index: usize) -> usize {
        match self.runs.binary_search_by(|run| run.end.cmp(&index)) {
            Ok(i) => i,
            Err(i) if i < self.runs.len() => i,
            _ => panic!("index out of bounds: the len is {} but the index is {}", self.len(), index)
        }
    }

    fn index_info(&self, index: usize) -> (usize, usize, usize) {
        match self.run_index(index) {
            0 => (0, 0, self.runs[0].end),
            index => (index, self.runs[index - 1].end + 1, self.runs[index].end),
        }
    }
}

impl<T: Eq> RleVec<T> {
    /// Appends an element to the back of this rle_vector.
    ///
    /// # Panics
    /// Panics if the number of elements in the vector overflows a usize.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    /// rle.push(1);
    /// assert_eq!(rle[0], 1);
    /// ```
    pub fn push(&mut self, value: T) {
        self.push_n(1, value);
    }

    /// Appends the same element n times to the back of this rle_vec.
    ///
    /// # Panics
    /// Panics if the number of elements in the vector overflows a usize.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::new();
    ///
    /// // Push 10 times a 2
    /// rle.push_n(10, 2);
    /// assert_eq!(rle[9], 2);
    /// ```
    pub fn push_n(&mut self, n: usize, value: T) {
        if n == 0 { return; }

        let end = match self.runs.last_mut() {
            Some(ref mut last) if last.value == value => return last.end += n,
            Some(ref mut last) => last.end + n,
            None => n - 1,
        };

        self.runs.push(InternalRun { value, end });
    }
}

impl<T: Eq + Clone> RleVec<T> {
    /// Constructs a new `RleVec<T>` from a Vec<T>.
    /// This consumes the `Vec<T>`
    ///
    /// # Examples
    /// ```
    /// # use rle_vec::RleVec;
    /// let rle = RleVec::from_slice(&[0, 0, 0, 1, 1, 99, 9]);
    /// assert_eq!(rle[3], 1);
    /// ```
    pub fn from_slice(slice: &[T]) -> RleVec<T> {
        slice.iter().cloned().collect()
    }

    /// Construct a `Vec<T>` from this `RleVec`.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let slice = &[0, 0, 0, 1, 1, 99, 9];
    /// let rle = RleVec::from_slice(slice);
    /// let vec = rle.to_vec();
    ///
    /// assert_eq!(vec.as_slice(), slice);
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }

    /// Modify the value at given index.
    ///
    /// This can result in the breaking of a run and therefore be an expensive operation.
    /// If the value is equal to the value currently present the complexity is
    /// **O(log n)**. But if the run needs to be broken the complexity increases to a worst case of
    /// **O((log n) + n)**.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 3]);
    ///
    /// assert_eq!(rle[2], 1);
    /// assert_eq!(rle.len(), 7);
    /// assert_eq!(rle.runs_len(), 3);
    ///
    /// rle.set(2, 3);
    /// assert_eq!(rle[2], 3);
    /// assert_eq!(rle.len(), 7);
    /// assert_eq!(rle.runs_len(), 5);
    /// ```
    pub fn set(&mut self, index: usize, value: T) {
        let (mut p, start, end) = self.index_info(index);

        if self.runs[p].value == value { return }

        // a size 1 run is replaced with the new value or joined with next or previous
        if end - start == 0 {
            // can we join the previous run?
            if p > 0 && self.runs[p - 1].value == value {
                self.runs.remove(p);
                self.runs[p - 1].end += 1;
                p -= 1;
            }
            // can we join the next run?
            if p < self.runs.len() - 1 && self.runs[p + 1].value == value {
                self.runs.remove(p);
                return;
            }
            // only one size-1 run in Rle replace its value
            self.runs[p].value = value;
            return;
        }

        // run size > 1, new value can split current run or maybe merge with previous or next
        if index == start {
            // compare to previous run
            if p > 0 {
                if self.runs[p - 1].value == value {
                    self.runs[p - 1].end += 1;
                } else {
                    self.runs.insert(p, InternalRun { value: value, end: start });
                }
            } else {
                self.runs.insert(0, InternalRun { value: value, end: 0 });
            }
        } else if index == end {
            // decrease current run length
            self.runs[p].end -= 1;

            // compare to next run
            if p < self.runs.len() - 1 && self.runs[p + 1].value == value {
            } else {
                self.runs.insert(p + 1, InternalRun {value: value, end: end});
            }
        } else {
            // split current run
            self.runs[p].end = index - 1;
            let v = self.runs[p].value.clone();
            // this might be more efficient using split_off, push and extend?
            // this implementation has complexity O((log n) + 2n)
            self.runs.insert(p + 1, InternalRun { value: value, end: index });
            self.runs.insert(p + 2, InternalRun { value: v, end: end });
        }
    }

    /// Removes and returns the element at position index, shifting all elements after it to the left.
    ///
    /// # Panics
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 1, 1, 4, 4]);
    ///
    /// assert_eq!(rle.remove(4), 2);
    /// assert_eq!(rle.runs_len(), 2);
    /// assert_eq!(rle.to_vec(), vec![1, 1, 1, 1, 1, 1, 4, 4]);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        let (p, start, end) = self.index_info(index);

        for run in self.runs[p..].iter_mut() {
            run.end -= 1;
        }

        // if size of the run is 1
        if end - start == 0 {
            let InternalRun { value, .. } = self.runs.remove(p); // `p + 1` become p
            // if value before and after are equal
            if p > 0 && self.runs_len() > 2 && self.runs[p - 1].value == self.runs[p].value {
                let after_end = self.runs[p].end;
                self.runs[p - 1].end = after_end;
                self.runs.remove(p);
            }
            value
        }
        else { self.runs[p].value.clone() }
    }

    /// Insert a value at the given index.
    ///
    /// Because the positions of the values after the inserted value need to be changed,
    /// the complexity of this function is **O((log n) + 2n)**.
    ///
    /// # Example
    /// ```
    /// # use rle_vec::RleVec;
    /// let mut rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 3]);
    ///
    /// assert_eq!(rle[2], 1);
    /// assert_eq!(rle.runs_len(), 3);
    ///
    /// rle.insert(2, 3);
    /// assert_eq!(rle[2], 3);
    /// assert_eq!(rle.runs_len(), 5);
    /// ```
    pub fn insert(&mut self, index: usize, value: T) {
        if index == self.len() {
            return self.push(value);
        }

        let (p, start, end) = self.index_info(index);
        // increment all run ends from position p
        for run in self.runs[p..].iter_mut() {
            run.end += 1;
        }

        if self.runs[p].value == value { return }

        // inserting value can split current run or maybe merge with previous or next
        if index == start {
            // compare to previous run
            if p > 0 && self.runs[p - 1].value == value {
                self.runs[p - 1].end += 1;
            } else {
                self.runs.insert(p, InternalRun { value, end: index });
            }
        } else {
            // split current run
            self.runs[p].end = index - 1;
            self.runs.insert(p + 1, InternalRun { value, end: index });
            let value = self.runs[p].value.clone();
            self.runs.insert(p + 2, InternalRun { value, end: end + 1 });
        }
    }
}

impl<T> Index<usize> for RleVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.runs[self.run_index(index)].value
    }
}

impl<T: Eq> FromIterator<T> for RleVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut rle = RleVec::new();
        for i in iter {
            rle.push(i);
        }
        rle
    }
}

impl<T: Eq> FromIterator<Run<T>> for RleVec<T> {
    fn from_iter<I: IntoIterator<Item=Run<T>>>(iter: I) -> Self {
        let mut rle = RleVec::new();
        for run in iter {
            rle.push_n(run.len, run.value);
        }
        rle
    }
}

/// Immutable `RelVec` iterator over values.
///
/// Can be obtained from the [`iter`](struct.RleVec.html#method.iter) method.
///
/// # Example
/// ```
/// # use rle_vec::RleVec;
/// let rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 3]);
///
/// let mut iterator = rle.iter();
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&2));
/// assert_eq!(iterator.next(), Some(&2));
/// assert_eq!(iterator.next(), Some(&3));
/// assert_eq!(iterator.next(), None);
/// ```
pub struct Iter<'a, T: 'a> {
    rle: &'a RleVec<T>,
    run_index: usize,
    index: usize,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rle.is_empty() || self.index == self.rle.len() {
            return None
        }
        let value = &self.rle.runs[self.run_index].value;
        self.index += 1;
        if self.index > self.rle.runs[self.run_index].end {
            self.run_index += 1;
        }
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = cmp::min(self.index + n, self.rle.len());
        self.run_index = if self.index < self.rle.len() {
            self.rle.run_index(self.index)
        } else {
            self.rle.runs.len() - 1
        };
        self.next()
    }
}

/// Immutable `RelVec` iterator over runs.
///
/// Can be obtained from the [`runs`](struct.RleVec.html#method.runs) method.
///
/// # Example
/// ```
/// # use rle_vec::{RleVec, Run};
/// let rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 3]);
///
/// let mut iterator = rle.runs();
/// assert_eq!(iterator.next(), Some(Run{ len: 4, value: &1 }));
/// assert_eq!(iterator.next(), Some(Run{ len: 2, value: &2 }));
/// assert_eq!(iterator.next(), Some(Run{ len: 1, value: &3 }));
/// assert_eq!(iterator.next(), None);
/// ```
pub struct Runs<'a, T:'a> {
    rle: &'a RleVec<T>,
    index: usize,
    last_end: usize,
}

impl<'a, T: 'a> Iterator for Runs<'a, T> {
    type Item = Run<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.rle.runs.len() {
            let &InternalRun { ref value, end } = self.rle.runs.index(self.index);
            let len = end - self.last_end + 1;
            self.index += 1;
            self.last_end = end + 1;
            Some(Run { len, value })
        }
        else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let mut rle = RleVec::<i64>::new();
        rle.push(1);
        rle.push(1);
        rle.push(1);
        rle.push(1);
        rle.push(2);
        rle.push(2);
        rle.push(2);
        rle.push(3);
        rle.push(3);
        rle.push(4);
        assert_eq!(rle.len(), 10);
        assert_eq!(rle.runs_len(), 4);

        rle.push_n(3, 4);
        assert_eq!(rle.len(), 13);
        assert_eq!(rle.runs_len(), 4);
        rle.push_n(3, 5);
        assert_eq!(rle.len(), 16);
        assert_eq!(rle.runs_len(), 5);
    }

    #[test]
    fn setting_values() {
        let mut rle = RleVec::<i64>::new();
        rle.push(1);
        rle.set(0, 10);
        assert_eq!(rle.len(), 1);
        assert_eq!(rle.runs_len(), 1);
        assert_eq!(rle[0], 10);

        let mut rle = RleVec::from_slice(&[1, 1, 1, 1, 2, 2, 2, 3, 3, 4]);

        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![1,1,1,1,2,2,2,3,3,4]);

        rle.set(2, 1);
        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![1,1,1,1,2,2,2,3,3,4]);
        rle.set(0, 1);
        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![1,1,1,1,2,2,2,3,3,4]);
        rle.set(0, 2);
        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![2,1,1,1,2,2,2,3,3,4]);
        rle.set(6, 5);
        rle.set(9, 2);
        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![2,1,1,1,2,2,5,3,3,2]);
        rle.set(2, 4);
        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![2,1,4,1,2,2,5,3,3,2]);
        rle.set(2, 1);
        assert_eq!((0..10).map(|i| rle[i]).collect::<Vec<_>>(), vec![2,1,1,1,2,2,5,3,3,2]);
        assert_eq!(rle.runs_len(), 6);
    }

    #[test]
    fn removing_values() {
        let mut rle = RleVec::from_slice(&[1, 1, 1, 1, 1, 2, 1, 1, 1, 4, 4, 3, 3]);
        assert_eq!(rle.len(), 13);
        assert_eq!(rle.runs_len(), 5);

        let value = rle.remove(5);
        assert_eq!(value, 2);
        assert_eq!(rle.len(), 12);
        assert_eq!(rle.runs_len(), 3);
        assert_eq!(rle.to_vec(), vec![1, 1, 1, 1, 1, 1, 1, 1, 4, 4, 3, 3]);

        let value = rle.remove(7);
        assert_eq!(value, 1);
        assert_eq!(rle.len(), 11);
        assert_eq!(rle.runs_len(), 3);
        assert_eq!(rle.to_vec(), vec![1, 1, 1, 1, 1, 1, 1, 4, 4, 3, 3]);

        let value = rle.remove(10);
        assert_eq!(value, 3);
        assert_eq!(rle.len(), 10);
        assert_eq!(rle.runs_len(), 3);
        assert_eq!(rle.to_vec(), vec![1, 1, 1, 1, 1, 1, 1, 4, 4, 3]);
    }

    #[test]
    fn inserting_values() {
        let mut v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
        let mut rle = RleVec::from_slice(&v);
        rle.insert(0,1);
        v.insert(0,1);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.len(),18);
        rle.insert(18,9);
        v.insert(18,9);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        rle.insert(19,10);
        v.insert(19,10);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);

        rle.insert(2,0);
        v.insert(2,0);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.runs_len(), 9);

        rle.insert(8,0);
        v.insert(8,0);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.runs_len(), 11);

        rle.insert(13,4);
        v.insert(13,4);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.runs_len(), 12);

        let v = vec![0,0,0,1,1,1,1,2,2,3];
        let mut rle: RleVec<_> = v.into_iter().collect();
        rle.set(1,2);
        assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), vec![0,2,0,1,1,1,1,2,2,3]);
        rle.insert(4,4);
        assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), vec![0,2,0,1,4,1,1,1,2,2,3]);
        rle.insert(7,1);
        assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), vec![0,2,0,1,4,1,1,1,1,2,2,3]);
        rle.insert(8,8);
        assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), vec![0,2,0,1,4,1,1,1,8,1,2,2,3]);
    }

    #[test]
    fn from_vec() {
        let v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
        let rle = RleVec::from_slice(&v);
        assert_eq!((0..v.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.len(),17);
    }

    #[test]
    fn iterators() {
        let v = vec![0,0,0,1,1,1,1,1,1,1,3,3,123,0,90,90,99];
        let rle = v.iter().cloned().collect::<RleVec<_>>();
        assert_eq!((0..v.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.len(), 17);

        assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), v);
        assert_eq!(RleVec::<i64>::new().iter().next(), None);

        let v2 = (0..100).collect::<Vec<usize>>();
        let rle2 = v2.iter().cloned().collect::<RleVec<_>>();
        assert_eq!(rle2.iter().cloned().collect::<Vec<_>>(), v2);
        assert_eq!(rle2.iter().skip(0).cloned().collect::<Vec<_>>(), v2);

        assert_eq!(rle2.iter().nth(0), Some(&0));
        assert_eq!(rle2.iter().nth(5), Some(&5));
        assert_eq!(rle2.iter().nth(99), Some(&99));
        assert_eq!(rle2.iter().nth(100), None);
        let mut it = rle2.iter();
        it.nth(0);
        assert_eq!(it.nth(0), Some(&1));

        assert_eq!(rle.iter().nth(3), Some(&1));
        assert_eq!(rle.iter().nth(14), Some(&90));
        assert_eq!(rle.iter().nth(15), Some(&90));

        assert_eq!(rle.iter().skip(2).next(), Some(&0));
        assert_eq!(rle.iter().skip(3).next(), Some(&1));

        assert_eq!(rle.iter().max(), Some(&123));
        assert_eq!(rle.iter().min(), Some(&0));
        assert_eq!(rle.iter().skip(13).max(), Some(&99));
        assert_eq!(rle.iter().skip(13).min(), Some(&0));
        assert_eq!(rle.iter().skip(13).take(2).max(), Some(&90));
        assert_eq!(rle.iter().skip(13).take(2).min(), Some(&0));

        //runiters
        assert_eq!(rle.runs().map(|r| r.value).collect::<Vec<_>>(), vec![&0,&1,&3,&123,&0,&90,&99]);
        assert_eq!(rle.runs().map(|r| r.len).collect::<Vec<_>>(), vec![3,7,2,1,1,2,1]);

        let mut copy = RleVec::new();
        for r in rle.runs() {
            copy.push_n(r.len, r.value.clone());
        }
        assert_eq!(copy.iter().cloned().collect::<Vec<_>>(), v);
        let copy2: RleVec<i32> = rle.runs().map(|r| Run { value: r.value.clone(), len: r.len }).collect();
        assert_eq!(copy2.iter().cloned().collect::<Vec<_>>(), v);
    }

    #[test]
    fn starts_ends() {
        let v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
        let rle = v.iter().cloned().collect::<RleVec<_>>();
        assert_eq!(rle.starts(), vec![0,3,10,12,13,14,16]);
        assert_eq!(rle.ends(),   vec![2,9,11,12,13,15,16]);

        let rle = RleVec::<i64>::new();
        assert!(rle.starts().is_empty());
        assert!(rle.ends().is_empty());
    }
}
