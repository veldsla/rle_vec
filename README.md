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

`Cargo bench` can be used to compare the real life difference of get/set/insert/remove operations on a `Vec` and `RleVec`. Warning, some test involves reallocations.

### Creation

```
rle_create_10_000_equal_values_from_iter       ... bench:       6,783 ns/iter (+/- 1,840)
vec_create_10_000_equal_values_from_iter       ... bench:      54,886 ns/iter (+/- 7,233)

rle_create_10_000_equal_values_from_slice      ... bench:      42,526 ns/iter (+/- 6,384)
vec_create_10_000_equal_values_from_slice      ... bench:       1,958 ns/iter (+/- 964)

rle_create_10_000_runs_of_10_values_from_iter  ... bench:      77,560 ns/iter (+/- 29,659)
vec_create_10_000_runs_of_10_values_from_iter  ... bench:     100,510 ns/iter (+/- 16,670)

rle_create_10_000_runs_of_10_values_from_slice ... bench:      43,341 ns/iter (+/- 10,864)
vec_create_10_000_runs_of_10_values_from_slice ... bench:       1,956 ns/iter (+/- 818)

rle_create_10_000_unique_values_from_iter      ... bench:      62,455 ns/iter (+/- 12,227)
vec_create_10_000_unique_values_from_iter      ... bench:       3,514 ns/iter (+/- 428)

rle_create_10_000_unique_values_from_slice     ... bench:      98,893 ns/iter (+/- 89,276)
vec_create_10_000_unique_values_from_slice     ... bench:       2,108 ns/iter (+/- 1,153)
```

### Access

```
rle_loop_10_000_equal_values      ... bench:     150,124 ns/iter (+/- 20,430)
vec_loop_10_000_equal_values      ... bench:       7,036 ns/iter (+/- 2,361)

rle_loop_10_000_runs_of_10_values ... bench:     153,809 ns/iter (+/- 20,454)
vec_loop_10_000_runs_of_10_values ... bench:       5,241 ns/iter (+/- 502)

rle_loop_10_000_unique_values     ... bench:     159,022 ns/iter (+/- 44,962)
vec_loop_10_000_unique_values     ... bench:      10,942 ns/iter (+/- 1,674)
```

### Insertion

Inserting data is very competitive and can be faster if no run breaking is
required. Indexing takes quite a big penalty, but mutable indexing is not that
bad.

```
rle_insert_middle_breaking_10_000_runs_of_10_values     ... bench:         233 ns/iter (+/- 41)
rle_insert_middle_non_breaking_10_000_runs_of_10_values ... bench:         245 ns/iter (+/- 29)
vec_insert_middle_10_000_runs_of_10_values              ... bench:      49,487 ns/iter (+/- 60,604)
```

### Set

```
rle_set_middle_10_000_equal_values            ... bench:          21 ns/iter (+/- 3)
vec_set_middle_10_000_equal_values            ... bench:           3 ns/iter (+/- 0)

rle_set_middle_10_000_runs_of_10_values       ... bench:          19 ns/iter (+/- 6)
vec_set_middle_10_000_runs_of_10_values       ... bench:           3 ns/iter (+/- 0)

rle_set_middle_10_000_unique_values           ... bench:          20 ns/iter (+/- 3)
vec_set_middle_10_000_unique_values           ... bench:           3 ns/iter (+/- 1)

rle_set_middle_same_value_10_000_equal_values ... bench:          18 ns/iter (+/- 2)
vec_set_middle_same_value_10_000_equal_values ... bench:           3 ns/iter (+/- 0)
```

### Deletion

```
rle_remove_middle_non_breaking_10_000_runs_of_10_values ... bench:      69,937 ns/iter (+/- 13,459)
rle_remove_middle_breaking_10_000_equal_values          ... bench:      59,973 ns/iter (+/- 10,970)
vec_remove_middle_10_000_runs_of_10_values              ... bench:      93,627 ns/iter (+/- 12,316)
```

*Remove* benches recreate the vector at each bench iteration. Substraction of the creation time get the real remove time.

```
rle_remove_middle_breaking_10_000_runs_of_10_values_sub_create     ... bench:     44,881 ns/iter
rle_remove_middle_non_breaking_10_000_equal_values                 ... bench:     49,605 ns/iter
vec_remove_middle_10_000_runs_of_10_values_sub_create              ... bench:     42,680 ns/iter
```

### To Vec

```
rle_to_vec_of_u8_10_000_equal_values  ... bench:     212,201 ns/iter (+/- 29,546)
rle_to_vec_of_u16_10_000_equal_values ... bench:     211,825 ns/iter (+/- 25,000)
rle_to_vec_of_u32_10_000_equal_values ... bench:     207,243 ns/iter (+/- 29,504)
rle_to_vec_of_u64_10_000_equal_values ... bench:     221,884 ns/iter (+/- 28,667)
```
