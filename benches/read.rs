
#![feature(test)]

extern crate test;
extern crate rle_vec;

use std::iter::FromIterator;
use std::iter::repeat;
use test::Bencher;
use rle_vec::RleVec;
use std::io::{Cursor, Read};


#[bench]
fn rle_read_100000_u8(b: &mut Bencher) {
    let zeros = repeat(0u8).take(10);
    let ones = repeat(1u8).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(100_000);

    let rle = RleVec::from_iter(iter);
    
    let mut buf = vec![0; 1024];
    b.iter(|| {
        let mut rle = rle.clone();
        loop {
            match rle.read(&mut buf) {
                Ok(n) if n == 0 => break,
                Ok(_) => {},
                Err(e) => panic!(e)
            }
        }
    });
}

#[bench]
fn cursor_vec_read_100000_u8(b: &mut Bencher) {
    let zeros = repeat(0u8).take(10);
    let ones = repeat(1u8).take(10);
    let iter = repeat(zeros.chain(ones)).flat_map(|x| x).take(100_000);

    let vec = Vec::from_iter(iter);
    
    let mut buf = vec![0; 1024];
    b.iter(|| {
        let mut cur = Cursor::new(vec.clone());
        loop {
            match cur.read(&mut buf) {
                Ok(n) if n == 0 => break,
                Ok(_) => {},
                Err(e) => panic!(e)
            }
        }
    });
}

