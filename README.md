# rle_vec

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
rle_vec = "0.2"
```

Then put this in your crate root:

```rust
extern crate rle_vec;
```

## RleVec
This crate provides `RleVec`, a vector like structure that stores runs of identical values coded
by the value and the number of repeats.

If your data consists of long stretches of identical values is can be beneficial to only store
the number of times each value occurs. This can result in significant space savings, but there
is a cost. Accessing an arbitrary index requires a binary search over the stored runs resulting
in a O(log n) complexity versus O(1) for a normal vector. Other complexities are in the table
where n is equal to the number of runs, not the length of a comparable Vec.

         |push|index   |set with breaking a run|set without breaking a run|insert with breaking a run|insert without breaking a run|
 --------|----|--------|-----------------------|--------------------------|--------------------------|-----------------------------|
 `RleVec`|O(1)|O(log n)|O((log n) + 2n)        |O(log n)                  |O((log n) + 2n)           |O((log n) + n)               |
 `Vec`   |O(1)|O(1)    |O(1)*                  |                          |O(n)                      |                             |

 * *Benchmarks show that setting `vec[idx] = value` is a lot slower than getting `vec[idx]`*

 The `RleVec` struct handles like a normal vector and supports a subset from the `Vec` methods.

## Examples:
 ```rust
 use rle_vec::RleVec;
 let mut rle = RleVec::new();
 rle.push(10); rle.push(10); rle.push(11);
 assert_eq!(rle[1], 10);
 assert_eq!(rle[2], 11);

 rle.insert(1,10);
 assert_eq!(rle.n_runs(), 2);
 rle.set(0,1);
 assert_eq!(rle.n_runs(), 3);
 ```

 `RleVec` can be constructed from `Iterators` and be iterated over just like a `Vec`.

 ```rust
 use rle_vec::RleVec;
 let v = vec![0,0,0,1,1,1,1,2,2,3,4,5,4,4,4];
 let mut rle: RleVec<_> = v.into_iter().collect();
 assert_eq!(rle.len(), 15);
 assert_eq!(rle.n_runs(), 7);

 assert_eq!(rle.iter().nth(10), Some(&4));
 ```

 An `RleVec` can be indexed like a regular vector, but not mutated. Use `RleVec::set` to change the
 value at an index.

 ```rust
 use rle_vec::RleVec;
 let v = vec![0,0,0,1,1,1,1,2,2,3];
 let mut rle: RleVec<_> = v.into_iter().collect();
 rle.set(1,2);
 rle.insert(4,4);
 assert_eq!(rle.iter().cloned().collect::<Vec<_>>(), vec![0,2,0,1,4,1,1,1,2,2,3]);

 ```
 `RleVec::set` and `RleVec::insert` require `T: Clone`.

 Not all methods implemented on `Vec` are implemented for `RleVec`. All methods returning a slice
 cannot work for `RleVec`.

## Benchmarks
 Cargo bench can be used to compare the real life difference of get/set/insert operations on a `Vec`
 and `RleVec`. The test data is a vector of length 5000 that contains 1000 runs of length 5.
 
 ```
running 10 tests
test create_1000_runs_length_5     ... bench:       9,990 ns/iter (+/- 857)
test create_vec_1000_runs_length_5 ... bench:       7,553 ns/iter (+/- 385)
test index_100_from_test           ... bench:       2,107 ns/iter (+/- 73)
test index_100_from_vec            ... bench:          59 ns/iter (+/- 2)
test insert_100_vec                ... bench:      50,248 ns/iter (+/- 1,945)
test insert_100_with_split         ... bench:      57,934 ns/iter (+/- 4,008)
test insert_100_without_split      ... bench:      27,416 ns/iter (+/- 801)
test set_100_vec                   ... bench:       7,453 ns/iter (+/- 286)
test set_100_with_split            ... bench:      42,654 ns/iter (+/- 1,055)
test set_100_without_split         ... bench:      12,094 ns/iter (+/- 234)
``` 

Inserting data is very competitive and can be faster if no run breaking is
required. Indexing takes quite a big penalty, but mutable indexing is not that
bad. 
