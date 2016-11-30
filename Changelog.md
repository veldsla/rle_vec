# Change Log
All notable changes to this project will be documented in this file.

## [0.2.0] - 2016-11-30
### Added
* Made a public type `Run`.
* `Iterators` for `Run` and `FromIterator<Run<T>`
* Added `min()` and `max()` to `RleVecIterator` that only look at run values.
  This is much more efficient than looping over each repeated value

## [0.1.0] - 2016-11-28
- Initial release

