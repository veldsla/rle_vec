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
        for v in vec.iter() {
            assert_eq!(*v, 0);
        }
    })
}

#[bench]
fn rle_to_vec_of_u16_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u16>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        for v in vec.iter() {
            assert_eq!(*v, 0);
        }
    })
}

#[bench]
fn rle_to_vec_of_u32_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u32>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        for v in vec.iter() {
            assert_eq!(*v, 0);
        }
    })
}

#[bench]
fn rle_to_vec_of_u64_10_000_equal_values(b: &mut Bencher) {
    let rle = RleVec::<u64>::from_iter(repeat(0).take(10_000));
    b.iter(|| {
        let vec = rle.to_vec();
        for v in vec.iter() {
            assert_eq!(*v, 0);
        }
    })
}
