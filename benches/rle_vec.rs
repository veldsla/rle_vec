#![feature(test)]

extern crate test;
extern crate rle_vec;

use test::Bencher;
use rle_vec::RleVec;

fn test_rle_vec() -> RleVec<u64> {
    let mut r = RleVec::new();
    for v in 1..1001 {
        for _ in 0..5 {
            r.push(v);
        }
    }
    r
}

fn test_vec() -> Vec<u64> {
    let mut r = Vec::new();
    for v in 1..1001 {
        for _ in 0 ..5 {
            r.push(v);
        }
    }
    r
}

#[bench]
fn create_1000_runs_length_5(b: &mut Bencher) {
    b.iter(|| {
        let r = test_rle_vec();
        assert_eq!(r.len(), 5000);
    });
}

#[bench]
fn create_vec_1000_runs_length_5(b: &mut Bencher) {
    b.iter(|| {
        let r = test_vec();
        assert_eq!(r.len(), 5000);
    });
}


#[bench]
fn index_100_from_test(b: &mut Bencher) {
    let r = test_rle_vec();
    b.iter(|| {
        for i in 800..900 {
            let _ = r[i];
        }
    });
}

#[bench]
fn index_100_from_vec(b: &mut Bencher) {
    let r = test_vec();
    b.iter(|| {
        for i in 800..900 {
            let _ = r[i];
        }
    });
}

#[bench]
fn set_100_with_split(b: &mut Bencher) {
    b.iter(|| {
        let mut r = test_rle_vec();
        let mut i = 2;
        let mut v = 0;
        for _ in 0..100 {
            r.set(i, v);
            i += 50;
            v += 10;
        }
        assert_eq!(r.len(), 5000);
        assert_eq!(r.runs(), 1200);
    });
}

#[bench]
fn set_100_without_split(b: &mut Bencher) {
    b.iter(|| {
        let mut r = test_rle_vec();
        let mut i = 2;
        let mut v = 1;
        for _ in 0..100 {
            r.set(i, v);
            i += 50;
            v += 10;
        }
        assert_eq!(r.len(), 5000);
        assert_eq!(r.runs(), 1000);
    });
}

#[bench]
fn set_100_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut r = test_vec();
        let mut i = 2;
        let mut v = 1;
        for _ in 0..100 {
            r[i] = v;
            i += 50;
            v += 10;
        }
    });
}

#[bench]
fn insert_100_with_split(b: &mut Bencher) {
    b.iter(|| {
        let mut r = test_rle_vec();
        let mut i = 2;
        let v = 0;
        for _ in 0..100 {
            r.insert(i, v);
            i += 50 + 1;
        }
        assert_eq!(r.len(), 5100);
        assert_eq!(r.runs(), 1200);
    });
}

#[bench]
fn insert_100_without_split(b: &mut Bencher) {
    b.iter(|| {
        let mut r = test_rle_vec();
        let mut i = 2;
        let mut v = 1;
        for _ in 0..100 {
            r.insert(i, v);
            i += 50 + 1;
            v += 10;
        }
        assert_eq!(r.len(), 5100);
        assert_eq!(r.runs(), 1000);
    });
}

#[bench]
fn insert_100_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut r = test_vec();
        let mut i = 2;
        let mut v = 0;
        for _ in 0..100 {
            r.insert(i, v);
            i += 50 + 1;
            v += 10;
        }
        assert_eq!(r.len(), 5100);
    });
}

