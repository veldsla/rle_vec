#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;

#[bench]
fn rle_to_vec_of_u8_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u8>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        assert_eq!(vec.len(), 10_000);
    })
}

#[bench]
fn rle_to_vec_of_u16_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u16>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        assert_eq!(vec.len(), 10_000);
    })
}

#[bench]
fn rle_to_vec_of_u16_10_000_unique_values(b: &mut Bencher) {
    let rle = RleVec::<u16>::from_iter(0 .. 10_000);
    b.iter(|| {
        let vec = rle.to_vec();
        assert_eq!(vec.len(), 10_000);
    })
}

#[bench]
fn rle_to_vec_of_u16_1000_runs_of_10_values(b: &mut Bencher) {
    let zeros = repeat(0).take(10);
    let ones = repeat(1).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(10_000);

    let rle = RleVec::from_iter(iter);

    b.iter(|| {
        let vec = rle.to_vec();
        assert_eq!(vec.len(), 10_000);
    })
}

#[bench]
fn rle_to_vec_of_u32_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u32>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        assert_eq!(vec.len(), 10_000);
    })
}

#[bench]
fn rle_to_vec_of_u64_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u64>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        assert_eq!(vec.len(), 10_000);
    })
}
