# Beyond The Basics of Rust Workshop

## Exercises

* Split into modules
  * Requires putting `pub` on the fields/methods that should be `pub`
  * Requires adding `mod` declarations and moving files to the right filename
* Use `Default` for `Reversi` instead of `new()`
* Add `Copy`, `PartialEq`, `Eq` to `TilePos` (more ergonomic)
  * Same for `Piece`
* Use `Self` instead of the struct name
  * Might not actually be an interesting exercise, so the code currently
    just uses `Self`
* Use iterators in `scores`
* The `rows` method on `Grid` exposes implementation details, replace it with
  a `fold` method or `for_each` method instead that takes a closure
* Use iterators to avoid an allocation in `adjacents`
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
