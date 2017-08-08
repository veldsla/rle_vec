#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_remove_middle_non_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    b.iter(|| {
        let zeros = repeat(0).take(10);
        let ones = repeat(1).take(10);
        let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

        let mut rle = RleVec::from_iter(iter);
        assert_eq!(rle.remove(5_000), 0);
    })
}

#[bench]
fn rle_remove_middle_breaking_1000_equal_values(b: &mut Bencher) {
    b.iter(|| {
        let mut rle = RleVec::from_iter(0..10_000);
        assert_eq!(rle.remove(5_000), 5_000);
    })
}

#[bench]
fn vec_remove_middle_1000_runs_of_10_values(b: &mut Bencher) {
    b.iter(|| {
        let zeros = repeat(0).take(10);
        let ones = repeat(1).take(10);
        let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

        let mut vec = Vec::from_iter(iter);
        assert_eq!(vec.remove(5_000), 0);
    })
}
