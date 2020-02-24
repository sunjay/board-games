# Beyond The Basics of Rust Workshop

## Exercises

* Split into modules
  * Requires putting `pub` on the fields/methods that should be `pub`
  * Requires adding `mod` declarations and moving files to the right filename
* Use `Default` for `Reversi` instead of `new()`
* Add `Copy`, `PartialEq`, `Eq` to `TilePos` (more ergonomic)
  * Same for `Piece`
  * Comment: Might not be a useful exercise to do more than once? Maybe we
    should implement `Copy` for `Piece` and fix up the code
* Use `Self` instead of the struct name
  * Comment: Might not actually be an interesting exercise, so the code
    currently just uses `Self`
* Use iterators in `scores`
* Implement `Iterator` for a struct `GridRows` and make `rows` returns `GridRows`
* The `rows` method on `Grid` exposes implementation details, replace it with
  a `fold` method or `for_each` method instead that takes a closure
* Use iterators to avoid an allocation in `adjacents`
  (TODO: adjacents was removed because it is not needed)
* Implement the `Add` trait for `TilePos` to make `adjacents` easier to read
  * Better abstraction: define a `Direction` struct or enum and impl `Add` for
    `TilePos` and `Direction` such that it returns `TilePos`
* `adjacents` can be written as two nested for loops too
  ```rust
  for drow in -1..=1 {
      for dcol in -1..=1 {
          if drow == 0 && dcol == 0 {
              continue;
          }

          // ...
      }
  }
  ```
* `adjacents` can also be written as a series of 8 if statements
* Implement `Display` for `Reversi`
* Implement `Display` and `Error` for `ParseError`
* Implement `From<io::Error>` for `ParseError`
  * Allows you to use the ? operator and get rid of `map_err`
* Return `Result<(), Box<dyn Error>>` from `main`
  * Mention `anyhow`
* Import variants using `use ParseError::*` to make code easier to read
* Implement `Display` for `TilePos` to avoid allocation in `TilePos::to_string`
* Implement `is_full` in terms of iterators (using `all`)
* Replace counter variables with `.enumerate()`
  * Comment: Is this a worthwhile exercise?
  * Comment: A person new to Rust might try this, so it might be worth it
  * Code currently just ses `.iter().enumerate()`
* Implement `valid_moves` and `is_valid_move` in terms of iterators
* Refactor `negamax_score` to remove duplication using a separate function that
  takes an iterator
