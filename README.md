# RleVec

A rust crate providing a vector like struct that stores data as runs of identical values.

If your data consists of long stretches of identical values it can be beneficial to only store
the number of times each value occurs. This can result in significant space savings, but there
is a cost. Accessing an arbitrary index requires a binary search over the stored runs resulting
in a `(log n)` complexity versus `O(1)` for a normal vector. Other complexities are in the table
where `n` is equal to the number of runs, not the length of a comparable `Vec`.

|        |push|index   |set with breaking a run|set without breaking a run|insert with breaking a run|insert without breaking a run|
|--------|----|--------|-----------------------|--------------------------|--------------------------|-----------------------------|
|`RleVec`|O(1)|O(log&nbsp;n)|O((log&nbsp;n)&nbsp;+&nbsp;2n)|O(log&nbsp;n)|O((log&nbsp;n)&nbsp;+&nbsp;2n)|O((log&nbsp;n)&nbsp;+&nbsp;n)|
|`Vec`|O(1)|O(1)|O(1)*| |O(n)| |

 \* *Benchmarks show that setting `vec[idx] = value` is a lot slower than getting `vec[idx]`*

 The `RleVec` struct handles like a normal vector and supports a subset from the `Vec` methods.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
rle_vec = "0.2"
```

and this to your crate root:

```rust
extern crate rle_vec;
```

## Examples:
```rust
use rle_vec::RleVec;

let mut rle = RleVec::new();

rle.push(10);
rle.push(10);
rle.push(11);
assert_eq!(rle[1], 10);
assert_eq!(rle[2], 11);

rle.insert(1, 10);
assert_eq!(rle.n_runs(), 2);

rle.set(0, 1);
assert_eq!(rle.n_runs(), 3);
```

`RleVec` can be constructed from `Iterators` and be iterated over just like a `Vec`.

```rust
use rle_vec::RleVec;

let v = vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 4, 5, 4, 4, 4];

let mut rle: RleVec<_> = v.into_iter().collect();
assert_eq!(rle.len(), 15);
assert_eq!(rle.n_runs(), 7);

assert_eq!(rle.iter().nth(10), Some(&4));
```

You can get the value at an index.

```rust
use rle_vec::RleVec;

let v = vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 3];
let mut rle: RleVec<_> = v.into_iter().collect();

rle.set(1, 2);
rle.insert(4, 4);

let v = rle.iter().cloned().collect::<Vec<_>>();
assert_eq!(v, vec![0, 2, 0, 1, 4, 1, 1, 1, 2, 2, 3]);
```

`RleVec::set` and `RleVec::insert` require `T: Clone`.

Not all methods implemented on `Vec` are implemented for `RleVec`. All methods returning a slice
cannot work for `RleVec`.

## Benchmarks

`Cargo bench` can be used to compare the real life difference of get/set/insert/remove operations on a `Vec` and `RleVec`.

Note that benches that needs mutable structs (set/insert/remove) will recreate the vector each time.

```
rle_loop_10_000_equal_values      ... bench:     161,405 ns/iter (+/- 17,573)
vec_loop_10_000_equal_values      ... bench:       7,653 ns/iter (+/- 616)

rle_loop_10_000_runs_of_10_values ... bench:     163,949 ns/iter (+/- 14,492)
vec_loop_10_000_runs_of_10_values ... bench:       5,612 ns/iter (+/- 612)

rle_loop_10_000_unique_values     ... bench:     166,887 ns/iter (+/- 19,187)
vec_loop_10_000_unique_values     ... bench:      11,307 ns/iter (+/- 1,175)


rle_insert_middle_breaking_10_000_runs_of_10_values     ... bench:      92,742 ns/iter (+/- 11,082)
rle_insert_middle_non_breaking_10_000_runs_of_10_values ... bench:      95,317 ns/iter (+/- 12,639)
vec_insert_middle_10_000_runs_of_10_values              ... bench:     103,501 ns/iter (+/- 17,365)


rle_remove_middle_breaking_10_000_unique_values         ... bench:     102,300 ns/iter (+/- 11,241)
rle_remove_middle_non_breaking_10_000_runs_of_10_values ... bench:      94,748 ns/iter (+/- 10,388)
vec_remove_middle_10_000_runs_of_10_values              ... bench:     104,008 ns/iter (+/- 13,896)


rle_set_middle_10_000_equal_values            ... bench:      46,219 ns/iter (+/- 5,695)
vec_set_middle_10_000_equal_values            ... bench:      58,268 ns/iter (+/- 7,255)

rle_set_middle_10_000_runs_of_10_values       ... bench:      94,895 ns/iter (+/- 13,902)
vec_set_middle_10_000_runs_of_10_values       ... bench:     104,953 ns/iter (+/- 10,561)

rle_set_middle_10_000_unique_values           ... bench:     108,793 ns/iter (+/- 47,550)
vec_set_middle_10_000_unique_values           ... bench:       3,952 ns/iter (+/- 635)

rle_set_middle_same_value_10_000_equal_values ... bench:      52,549 ns/iter (+/- 46,953)
vec_set_middle_same_value_10_000_equal_values ... bench:      57,428 ns/iter (+/- 6,448)


rle_to_vec_of_u8_10_000_equal_values  ... bench:     225,858 ns/iter (+/- 21,944)
rle_to_vec_of_u16_10_000_equal_values ... bench:     222,045 ns/iter (+/- 27,802)
rle_to_vec_of_u32_10_000_equal_values ... bench:     215,114 ns/iter (+/- 24,490)
rle_to_vec_of_u64_10_000_equal_values ... bench:     256,898 ns/iter (+/- 158,647)
```

Inserting data is very competitive and can be faster if no run breaking is
required. Indexing takes quite a big penalty, but mutable indexing is not that
bad.
