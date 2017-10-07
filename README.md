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

 The `RleVec` struct handles like a normal vector and supports a subset from the `Vec` methods.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
rle_vec = "0.3"
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
assert_eq!(rle.runs_len(), 2);

rle.set(0, 10);
assert_eq!(rle.runs_len(), 3);
```

`RleVec` can be constructed from `Iterators` and be iterated over just like a `Vec`.

```rust
use rle_vec::RleVec;

let v = vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 4, 5, 4, 4, 4];

let mut rle: RleVec<_> = v.into_iter().collect();
assert_eq!(rle.len(), 15);
assert_eq!(rle.runs_len(), 7);

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

## Intended use
 * Allocate gigantic vectors with a starting value and (randomly) update
   positions under the assumption the data is going to remain sparse. The
update step will be slower than using a `Vec`.
 * Obviously the savings are bigger when the size of `T` in `RleVec<T>` is
   large.
 * If you want to reduce an in memory structure before for example serializing.

## Benchmarks

`Cargo bench` can be used to compare the real life difference of
get/set/insert/remove operations on a `Vec` and `RleVec`. Warning, some test
involves reallocations.

### Creation

Create appears to be fastest from slice, unless the data is very sparse.
```
rle_create_1000_runs_of_10_values_from_iter  ... bench:      19,561 ns/iter (+/- 342)
vec_create_1000_runs_of_10_values_from_iter  ... bench:      22,221 ns/iter (+/- 582)

rle_create_1000_runs_of_10_values_from_slice ... bench:       6,076 ns/iter (+/- 324)
vec_create_1000_runs_of_10_values_from_slice ... bench:         894 ns/iter (+/- 45)

rle_create_10_000_equal_values_from_iter     ... bench:          15 ns/iter (+/- 1)
vec_create_10_000_equal_values_from_iter     ... bench:       7,683 ns/iter (+/- 547)

rle_create_10_000_equal_values_from_slice    ... bench:       4,490 ns/iter (+/- 57)
vec_create_10_000_equal_values_from_slice    ... bench:         898 ns/iter (+/- 48)

rle_create_10_000_unique_values_from_iter    ... bench:      25,510 ns/iter (+/- 789)
vec_create_10_000_unique_values_from_slice   ... bench:         891 ns/iter (+/- 47)

rle_create_10_000_unique_values_from_slice   ... bench:      25,936 ns/iter (+/- 278)
vec_create_10_000_unique_values_from_iter    ... bench:         921 ns/iter (+/- 25)
```

### Access
Random access takes the binary search penalty, but iterating performs reasonable.
```
rle_iterate_1000_runs_of_10_values       ... bench:      19,773 ns/iter (+/- 481)
vec_iterate_1000_runs_of_10_values       ... bench:       4,981 ns/iter (+/- 2,171)

rle_iterate_10_000_equal_values          ... bench:      20,878 ns/iter (+/- 538)
vec_iterate_10_000_equal_values          ... bench:       5,149 ns/iter (+/- 124)

rle_iterate_10_000_unique_values         ... bench:      20,130 ns/iter (+/- 340)
vec_iterate_10_000_unique_values         ... bench:       7,784 ns/iter (+/- 127)

rle_random_access_1000_runs_of_10_values ... bench:      34,999 ns/iter (+/- 632)
vec_random_access_1000_runs_of_10_values ... bench:         499 ns/iter (+/- 11)

```

### Insertion

Inserting data is competitive and can be faster if no run breaking is
required.
```
rle_insert_runmids_breaking_1000_runs_of_10_values     ... bench:     308,797 ns/iter (+/- 23,860)
rle_insert_runmids_non_breaking_1000_runs_of_10_values ... bench:     171,507 ns/iter (+/- 2,669)
vec_insert_runmids_1000_runs_of_10_values              ... bench:     191,124 ns/iter (+/- 5,439)
```

### Set
Mutable indexing can have very different outcomes. Minimum cost is the binary
search, but depending on the inserted value `Runs` can be merged or split.
```
test rle_set_runmids_breaking_1000_runs_of_10_values     ... bench:     177,418 ns/iter (+/- 2,718)
test rle_set_runmids_non_breaking_1000_runs_of_10_values ... bench:      34,844 ns/iter (+/- 528)
test rle_set_runs_merging_1000_runs                      ... bench:      97,703 ns/iter (+/- 1,521)
test vec_set_runmids_1000_runs_of_10_values              ... bench:         908 ns/iter (+/- 11)
test vec_set_runs_merging_1000_runs                      ... bench:         785 ns/iter (+/- 27)
```

### Deletion
Removing values can have very different outcomes. Minimum cost is the binary
search, but depending on the removed value `Runs` can be merged or split. But
remove is also a lot more expensive for a `Vec`.
```
test rle_remove_runmids_non_breaking_1000_runs_of_10_values ... bench:     184,023 ns/iter (+/- 5,367)
test rle_remove_runs_merging_1000_runs                      ... bench:     182,233 ns/iter (+/- 5,122)
test vec_remove_runmids_1000_runs_of_10_values              ... bench:     270,981 ns/iter (+/- 6,258)
test vec_remove_runs_merging_1000_runs                      ... bench:      66,948 ns/iter (+/- 1,460)
```
