# Change Log
All notable changes to this project will be documented in this file.

## [0.2.2] - 2017-02-01
### Fixed
* Removed a couple of println! statements.
* Added crate category

## [0.2.1] - 2016-12-06
### Added
* Implemented `nth` on `RleVecIterator`. The will speed up iterator functions like
  `nth` and `skip` is large runs are present
* Derived `RustcEncodable` and `RustcDecodable` on the structs so the data can be serialized

## [0.2.0] - 2016-11-30
### Added
* Made a public type `Run`.
* `Iterators` for `Run` and `FromIterator<Run<T>`
* Added `min()` and `max()` to `RleVecIterator` that only look at run values.
  This is much more efficient than looping over each repeated value

## [0.1.0] - 2016-11-28
- Initial release

