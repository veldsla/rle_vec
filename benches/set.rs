#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_set_runmids_non_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec: Vec<_> = iter.collect();
    let mut rle = RleVec::from(&vec[..]);

    b.iter(|| {
        let mut i = 5;
        let mut val = 0;
        for _ in 0..1000 {
            rle.set(i, val);
            i += 10;
            val = if val == 0 { 1 } else { 0 };
        }
        assert_eq!(rle.len(), 10_000);
        assert_eq!(rle.runs_len(), 1_000);
    })
}

#[bench]
fn rle_set_runmids_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut rle = RleVec::from(&vec[..]);
        let mut i = 5;
        let mut val = 1;
        for _ in 0..1000 {
            rle.set(i, val);
            i += 10;
            val = if val == 0 { 1 } else { 0 };
        }
        assert_eq!(rle.len(), 10_000);
        assert_eq!(rle.runs_len(), 3_000);
    })
}

#[bench]
fn rle_set_runs_merging_1000_runs(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(1);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(5_500);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut rle = RleVec::from(&vec[..]);
        let mut i = 10;
        for _ in 0..500 {
            rle.set(i, 0);
            i += 11;
        }
        assert_eq!(rle.len(), 5_500);
        assert_eq!(rle.runs_len(), 1);
    })
}

#[bench]
fn vec_set_runmids_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let mut vec = Vec::from_iter(iter);

    b.iter(|| {
        let mut i = 5;
        let mut val = 0;
        for _ in 0..1000 {
            vec[i] = val;
            i += 10;
            val = if val == 0 { 1 } else { 0 };
        }
        assert_eq!(vec.len(), 10_000);
    })
}

#[bench]
fn vec_set_runs_merging_1000_runs(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(1);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(5_500);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut vec = vec.clone();
        let mut i = 10;
        for _ in 0..500 {
            vec[i] = 0;
            i += 11;
        }
        assert_eq!(vec.len(), 5_500);
    })
}
