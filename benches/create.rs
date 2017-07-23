#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_create_10_000_equal_values_from_slice(b: &mut Bencher) {
    b.iter(|| {
        RleVec::from_slice(&[0; 10_000]);
    })
}

#[bench]
fn vec_create_10_000_equal_values_from_slice(b: &mut Bencher) {
    b.iter(|| {
        let _: Vec<_> = [0; 10_000][..].into();
    })
}

#[bench]
fn rle_create_10_000_equal_values_from_iter(b: &mut Bencher) {
    b.iter(|| {
        RleVec::from_iter(repeat(0).take(10_000));
    })
}

#[bench]
fn vec_create_10_000_equal_values_from_iter(b: &mut Bencher) {
    b.iter(|| {
        Vec::from_iter(repeat(0).take(10_000));
    })
}

#[bench]
fn rle_create_10_000_runs_of_10_values_from_iter(b: &mut Bencher) {
    b.iter(|| {
        let zeros = repeat(0).take(10);
        let ones = repeat(1).take(10);
        let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

        RleVec::from_iter(iter);
    })
}
#[bench]
fn vec_create_10_000_runs_of_10_values_from_iter(b: &mut Bencher) {
    b.iter(|| {
        let zeros = repeat(0).take(10);
        let ones = repeat(1).take(10);
        let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

        Vec::from_iter(iter);
    })
}

#[bench]
fn rle_create_10_000_runs_of_10_values_from_slice(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let vec = Vec::from_iter(iter);
    let slice = vec.as_slice();

    b.iter(|| {
        RleVec::from_slice(&slice);
    })
}
#[bench]
fn vec_create_10_000_runs_of_10_values_from_slice(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let vec = Vec::from_iter(iter);
    let slice = vec.as_slice();

    b.iter(|| {
        Vec::from(&slice[..]);
    })
}
