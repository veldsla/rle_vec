#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_insert_middle_non_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut rle = RleVec::from(&vec[..]);
        let middle_value = rle[5_000];
        rle.insert(5_000, middle_value);
    })
}

#[bench]
fn rle_insert_middle_breaking_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);
    let vec: Vec<_> = iter.collect();

    b.iter(|| {
        let mut rle = RleVec::from(&vec[..]);
        rle.insert(5_000, 424242);
    })
}

#[bench]
fn vec_insert_middle_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let tmp_vec = Vec::from_iter(iter);

    b.iter(|| {
        let mut vec = tmp_vec.clone();
        let middle_value = vec[5_000];
        vec.insert(5_000, middle_value);
    })
}
