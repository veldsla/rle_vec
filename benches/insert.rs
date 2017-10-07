#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_insert_runmids_non_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut rle = RleVec::from(&vec[..]);
        let mut i = 5;
        let mut val = 0;
        for _ in 0..1000 {
            rle.insert(i, val);
            i += 11;
            val = if val == 0 { 1 } else { 0 };
        }
        assert_eq!(rle.len(), 11_000);
        assert_eq!(rle.runs_len(), 1000);
    })
}

#[bench]
fn rle_insert_runmids_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut rle = RleVec::from(&vec[..]);
        let mut i = 5;
        let mut val = 1;
        for _ in 0..1000 {
            rle.insert(i, val);
            i += 11;
            val = if val == 0 { 1 } else { 0 };
        }
        assert_eq!(rle.len(), 11_000);
        assert_eq!(rle.runs_len(), 3000);
    })
}

#[bench]
fn vec_insert_runmids_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec = Vec::from_iter(iter);

    b.iter(|| {
        let mut vec = vec.clone();
        let mut i = 5;
        let mut val = 0;
        for _ in 0..1000 {
            vec.insert(i, val);
            i += 11;
            val = if val == 0 { 1 } else { 0 };
        }
        assert_eq!(vec.len(), 11_000);
    })
}
