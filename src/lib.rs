use std::ops::Index;

pub struct RleVec<T> where T: Eq {
    pub runs: Vec<Run<T>>
}


pub struct Run<T> where T: Eq {
    pub length: usize,
    pub value: T
}

impl<T> RleVec<T> where T: Eq {
    /// Constructs a new `RleVec<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rlevec::RleVec;
    ///
    /// let rle = RleVec::new();
    /// ```
    pub fn new() -> RleVec<T> {
        RleVec {runs: Vec::new()}
    }

    /// Constructs a new `RleVec<T>` from a Vec<T>.
    /// This consumes the `Vec<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// let v = vec![0,0,0,1,1,99,9];
    /// let rle = RleVec::from_vec(v);
    /// assert_eq!(rle[3],1);
    /// ```
    pub fn from_vec(v: Vec<T>) -> RleVec<T> {
        let mut rle = RleVec::<T>::new();
        if v.len() == 0 {
            return rle;
        }

        for value in v {
            if let Some(run) = rle.runs.last_mut() {
                if run.value == value {
                    run.length += 1;
                    continue;
                }
            }
            let r = Run { value: value, length: 1};
            rle.runs.push(r);
        }
        rle
    }

    pub fn add_run(&mut self, run: Run<T>) {
        self.runs.push(run);
    }

    /// Return the total run-length
    pub fn len(&self) -> usize {
        self.runs.iter().map(|r| r.length).fold(0, |x, sum| x+sum)
    }

    pub fn starts(&self) -> Vec<usize> {
        unimplemented!();
    }

    pub fn ends(&self) -> Vec<usize> {
        unimplemented!();
    }
}

impl<T> Index<usize> for RleVec<T> where T: Eq {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        let mut p = 0;
        for r in &self.runs {
            p += r.length ;
            if p - 1  >= index {
                return &r.value;
            }
        }
        panic!("RleVec index out of bounds: the len is {} but the index is {}", self.len(), index);
    }
}


#[test]
fn basic_usage() {
    let mut rle = RleVec::<i64>::new();
    let r1 = 
    rle.add_run(Run {value: 1, length: 10});
    rle.add_run(Run {value: 2, length: 5});
    rle.add_run(Run {value: -1, length: 7});

    assert_eq!(rle.len(), 22);
    assert_eq!(rle[1], 1);
    assert_eq!(rle[2], 1);
    assert_eq!(rle[10], 2);
    assert_eq!(rle[21], -1);
    //panics with out of bounds:
    //assert_eq!(rle[22], -1);
}

#[test]
fn from_vec() {
    let v = vec![0,0,0,1,1,1,1,1,1,1,3,3,1,0,99,99,9];
    let rle = RleVec::from_vec(v);

    assert_eq!(rle.len(),17);
    assert_eq!(rle[0],0);
    assert_eq!(rle[2],0);
    assert_eq!(rle[3],1);
    assert_eq!(rle[10],3);
    assert_eq!(rle[16],9);
    assert_eq!(rle.len(),17);

}

