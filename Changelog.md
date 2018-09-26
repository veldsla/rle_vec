# Change Log
All notable changes to this project will be documented in this file.

## [0.4.0] - 2018-9-26
### Feature
* Make Serde an optional feature. It take quite some time to compile which is not always necessary.

## [0.3.1] - 2017-10-27
### Fixed
* Really use Serde as was promised in 0.3

## [0.3] - 2017-10-11
Thanks to a huge pull-request by @Kerollmops the RleVec know provides a more complete set of
feaures. @Kerollmops kindly offered to join the project and is now a co-maintainer.

### Breaking changes
 * `RleVec::n_runs` is now called `RleVec::runs_len`
 * Deprecated rustc_serialize in favor of Serde
### New features
 * Added methods `remove`, `clear`, `to_vec` to `RleVec`
 * The RleVec Iterator now implements the `ExactSizeIterator` and `DoubleEndedIterator` traits
 * Implemented traits `Default`, `Write`, `Extend` and `Into<Vec<T>>` for `RleVec`
 * Greatly increased the number of benchmarks
 * Reorganised and increased number of tests

## [0.2.2] - 2017-02-01
### Fixed
* Removed a couple of println! statements.
* Added crate category

## [0.2.1] - 2016-12-06
### Added
* Implemented `nth` on `RleVecIterator`. The will speed up iterator functions like
  `nth` and `skip` if large runs are present
* Derived `RustcEncodable` and `RustcDecodable` on the structs so the data can be serialized

## [0.2.0] - 2016-11-30
### Added
* Made a public type `Run`.
* `Iterators` for `Run` and `FromIterator<Run<T>`
* Added `min()` and `max()` to `RleVecIterator` that only look at run values.
  This is much more efficient than looping over each repeated value

## [0.1.0] - 2016-11-28
- Initial release

