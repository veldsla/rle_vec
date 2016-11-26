use std::iter::FromIterator;
use std::ops::Index;

#[derive(Debug)]
pub struct RleVec<T> where T: Eq + Clone {
    runs: Vec<Run<T>>,
}

#[derive(Debug)]
struct Run<T> where T: Eq + Clone {
    value: T,
    end: usize
}

impl<T> RleVec<T> where T: Eq + Clone {
    /// Constructs a new empty `RleVec<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rle_vec::RleVec;
    ///
    /// let mut rle = RleVec::<u64>::new();
    /// rle.push(10);
    /// ```
    pub fn new() -> RleVec<T> {
        RleVec { runs: Vec::new() }
    }

    /// Constructs a new empty `RleVec<T>` with capacity for the number of runs. Choosing this value
    /// requires knowledge about the composition of the data that is going to be inserted.
    pub fn with_capacity(capacity: usize) -> RleVec<T> {
        RleVec { runs: Vec::with_capacity(capacity) }
    }

    /// Constructs a new `RleVec<T>` from a Vec<T>.
    /// This consumes the `Vec<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use rle_vec::RleVec;
    /// 
    /// let v = vec![0,0,0,1,1,99,9];
    /// let rle = RleVec::from_vec(v);
    /// assert_eq!(rle[3],1);
    /// ```
    pub fn from_vec(v: Vec<T>) -> RleVec<T> {
        let mut rle = RleVec::<T>::new();

        for value in v {
            rle.push(value)
        }
        rle
    }

    /// Add single value to the rle-vector
    pub fn push(&mut self, value: T) {
        let end = if let Some(last) = self.runs.last_mut() {
            if last.value == value {
                last.end += 1;
                return;
            }
            last.end + 1
        } else {
            0
        };

        self.runs.push(Run { value: value, end: end });
    }
    
    /// Modify the RleVec at index. This can result in the breaking of a run and therefore be an
    /// expensive oparation. If the value is equal to the value currently present the complexity is
    /// O(log n). But if the run needs to be broken the complexity increases to a worst case of
    /// O((log n) + n)
    pub fn set(&mut self, index: usize, value: T) {
        let (mut p, start, end) = self.index_info(index);
        //no change early return
        if self.runs[p].value == value {
            return;
        }

        //a size 1 run is replaced with the new value or joined with next or previous
        if end - start == 0 {
            //can we join the previous run?
            if p > 0 && self.runs[p-1].value == value {
                self.runs.remove(p);
                self.runs[p-1].end += 1;
                p -= 1;
            }
            // can we join the next run?
            if p < self.runs.len() - 1 && self.runs[p+1].value == value {
                self.runs.remove(p);
                return;
            }
            //only one size-1 run in Rle replace its value
            self.runs[p].value = value;
            return;
        }

        //run size > 1, new value can split current run or maybe merge with previous or next
        if index == start {
            //compare to previous run
            if p > 0 {
                if self.runs[p-1].value == value {
                    self.runs[p-1].end += 1;
                } else {
                    self.runs.insert(p, Run { value: value, end: start });
                }
            } else {
                self.runs.insert(0, Run { value: value, end: 0 });
            }
        } else if index == end {
            //decrease current run length
            self.runs[p].end -= 1;
            
            //compare to next run
            if p < self.runs.len() - 1 && self.runs[p+1].value == value {
            } else {
                self.runs.insert(p+1, Run {value: value, end: end});
            }
        } else {
            //split current run
            self.runs[p].end -= index - start + 1;
            let v = self.runs[p].value.clone();
            //this might be more efficient using split_off, push and extend?
            //this implementation has complexity O((log n) + 2n)
            self.runs.insert(p + 1, Run { value: value, end: index });
            self.runs.insert(p + 2, Run { value: v, end: end });
        }
    }

    /// Insert a value in the RleVec at index. Because the positions of the values after the
    /// inserted value need to be changed the complexity of this function is O((log n) + 2n)
    pub fn insert(&mut self, index: usize, value: T) {
        if index == self.len() {
            self.push(value);
            return;
        }

        let (p, start, end) = self.index_info(index);
        //increment all run ends from position p
        for r in self.runs[p..].iter_mut() {
            r.end += 1;
        }
        
        //if value is the same as in run were done
        if self.runs[p].value == value {
            return;
        }

        //run size > 1, new value can split current run or maybe merge with previous or next
        if index == start {
            //compare to previous run
            if p > 0 && self.runs[p-1].value == value {
                self.runs[p-1].end += 1;
            } else {
                self.runs.insert(p, Run { value: value, end: index });
            }
        } else if index == end {
            //compare to next run
            if p < self.runs.len() - 1 && self.runs[p+1].value == value {
                self.runs[p+1].end -= 1;
            } else {
                self.runs.insert(p+1, Run {value: value, end: index});
            }
        } else {
            //split current run
            self.runs[p].end -= index - start + 2;
            let v = self.runs[p].value.clone();
            self.runs.insert(p + 1, Run { value: value, end: index });
            self.runs.insert(p + 2, Run { value: v, end: end + 1 });
        }
    }

    /// Return the total run-length O(1)
    pub fn len(&self) -> usize {
         match self.runs.last() {
                None => 0,
                Some(r) => r.end + 1
       }
    }

    pub fn n_runs(&self) -> usize {
        self.runs.len()
    }

    pub fn starts(&self) -> Vec<usize> {
        let mut res = vec![0];
        res.extend(self.runs.iter().map(|e| e.end - 1));
        res
    }

    pub fn ends(&self) -> Vec<usize> {
        self.runs.iter().map(|r| r.end).collect()
    }

    pub fn iter<'a>(&'a self) -> RleVecIterator<'a, T> {
        RleVecIterator {
            rle: &self,
            pos: 0,
            index: 0,
            remaining: if self.runs.len() > 0 { self.runs[0].end } else { 0 }
        }
    }

    fn index_pos(&self, index: usize) -> usize {
        match self.runs.binary_search_by(|probe| probe.end.cmp(&index)) {
            Ok(p) => p,
            Err(p) if p < self.runs.len() => p,
            _ => panic!("RleVec index out of bounds: the len is {} but the index is {}", self.len(), index)

        }
    }

    fn index_info(&self, index: usize) -> (usize, usize, usize) {
        let p = self.index_pos(index);
        match p {
            0 => (p, 0, self.runs[0].end),
            _ => (p, self.runs[p-1].end + 1, self.runs[p].end),
        }
    }
}

impl<T> Index<usize> for RleVec<T> where T: Eq + Clone {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        let p = self.index_pos(index);
        &self.runs[p].value
    }
}

pub struct RleVecIterator<'a, T: 'a + Eq + Clone> {
    rle: &'a RleVec<T>,
    pos: usize,
    index: usize,
    remaining: usize
}

impl<'a, T: 'a +  Eq + Clone> Iterator for RleVecIterator<'a, T> {
    // we will be counting with usize
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.remaining == 0 {
            if self.pos < self.rle.runs.len() {
                self.pos += 1;
                self.remaining = self.index - self.rle.runs[self.pos].end;
            } else {
                return None;
            }
        }
        None
    }
}

impl<T> FromIterator<T> for RleVec<T> where T: Eq + Clone {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = RleVec::new();
        for i in iter {
            c.push(i);
        }
        c
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
        assert_eq!(rle.n_runs(), 4);
    }

    #[test]
    fn setting_values() {
        let mut rle = RleVec::<i64>::new();
        rle.push(1);
        rle.set(0, 10);
        assert_eq!(rle.len(), 1);
        assert_eq!(rle.n_runs(), 1);
        assert_eq!(rle[0], 10);

        let mut rle = RleVec::from_vec(vec![1,1,1,1,2,2,2,3,3,4]);

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
        assert_eq!(rle.n_runs(), 6);

    }

    #[test]
    fn inserting_values() {
        let mut v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
        let mut rle = RleVec::from_vec(v.clone());
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
        assert_eq!(rle.n_runs(),9);

        rle.insert(8,0);
        v.insert(8,0);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.n_runs(),11);

        rle.insert(13,4);
        v.insert(13,4);
        assert_eq!((0..rle.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.n_runs(),12);
        
        
    }

    #[test]
    fn from_vec() {
        let v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
        let rle = RleVec::from_vec(v.clone());
        assert_eq!((0..v.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.len(),17);
    }

    #[test]
    fn iterators() {
        let v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
        let rle = v.iter().cloned().collect::<RleVec<_>>();
        assert_eq!((0..v.len()).map(|i| rle[i]).collect::<Vec<_>>(), v);
        assert_eq!(rle.len(),17);
    }
}
