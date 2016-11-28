# rle_vec

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
rle_vec = "0.1"
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

 *\*Benchmarks show that setting `vec[idx] = value` is a lot slower than getting `vec[idx]`*

 The `RleVec` struct handles like a normal vector and supports a subset from the `Vec` methods.

 # Examples:
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
