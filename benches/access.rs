#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_iterate_10_000_unique_values(b: &mut Bencher) {
    let rle = RleVec::from_iter(0..10_000);
    b.iter(|| {
        for (i, v) in rle.iter().enumerate() {
            assert_eq!(i, *v);
        }
    })
}

#[bench]
fn vec_iterate_10_000_unique_values(b: &mut Bencher) {
    let vec = Vec::from_iter(0..10_000);
    b.iter(|| {
        for (i, v) in vec.iter().enumerate() {
            assert_eq!(i, *v);
        }
    })
}

#[bench]
fn rle_iterate_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        for v in rle.iter() {
            assert_eq!(*v, 0);
        }
    })
}

#[bench]
fn vec_iterate_10_000_equal_values(b: &mut Bencher) {
    let vec = Vec::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        for v in &vec {
            assert_eq!(*v, 0);
        }
    })
}

#[bench]
fn rle_iterate_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let rle = RleVec::from_iter(iter);
    b.iter(|| {
        for v in rle.iter() {
            assert!(*v == 0 || *v == 1); // ugly
        }
    })
}

#[bench]
fn vec_iterate_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let vec = Vec::from_iter(iter);
    b.iter(|| {
        for v in &vec {
            assert!(*v == 0 || *v == 1); // ugly
        }
    })
}

#[bench]
fn rle_random_access_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let rle = RleVec::from_iter(iter);
    let len = rle.len();
    b.iter(|| {
        let mut i = 5;
        while i < len {
            let _ = rle[i];
            i += 10;
        }
    })
}

#[bench]
fn vec_random_access_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let vec = Vec::from_iter(iter);
    let len = vec.len();
    b.iter(|| {
        let mut i = 5;
        while i < len {
            let _ = vec[i];
            i += 10;
        }
    })
}
