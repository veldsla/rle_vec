#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_set_middle_10_000_unique_values(b: &mut Bencher) {
    b.iter(|| {
        let mut rle = RleVec::from_iter(0..10_000);
        rle.set(5_000, 424242);
    })
}

#[bench]
fn vec_set_middle_10_000_unique_values(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::from_iter(0..10_000);
        vec[5_000] = 424242;
    })
}

#[bench]
fn rle_set_middle_10_000_equal_values(b: &mut Bencher) {
    b.iter(|| {
        let mut rle = RleVec::from_iter(repeat(0).take(10_000));
        rle.set(5_000, 424242);
    })
}

#[bench]
fn vec_set_middle_10_000_equal_values(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::from_iter(repeat(0).take(10_000));
        vec[5_000] = 424242;
    })
}

#[bench]
fn rle_set_middle_10_000_runs_of_10_values(b: &mut Bencher) {
    b.iter(|| {
        let zeros = repeat(0).take(10);
        let ones = repeat(1).take(10);
        let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

        let mut rle = RleVec::from_iter(iter);
        rle.set(5_000, 424242);
    })
}

#[bench]
fn vec_set_middle_10_000_runs_of_10_values(b: &mut Bencher) {
    b.iter(|| {
        let zeros = repeat(0).take(10);
        let ones = repeat(1).take(10);
        let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

        let mut vec = Vec::from_iter(iter);
        vec[5_000] = 424242;
    })
}

#[bench]
fn rle_set_middle_same_value_10_000_equal_values(b: &mut Bencher) {
    b.iter(|| {
        let mut rle = RleVec::from_iter(repeat(0).take(10_000));
        rle.set(5_000, 0);
    })
}

#[bench]
fn vec_set_middle_same_value_10_000_equal_values(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::from_iter(repeat(0).take(10_000));
        vec[5_000] = 0;
    })
}
