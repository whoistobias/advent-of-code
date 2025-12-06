# Things I learned from this advent of code

- You can use `&str.lines()` instead of `&str.split("\n")` since it handles different line endings.
- An easy way to handle integer over/underflow is by using the `saturated_foo` functions.
- The `format!` macro can pad strings if you if like so: `format!("{:<length$}", string);`
- You can't have reverse range literals (`15..0`), but you can do them by reversing a normal range e.g. (`(15..0).rev()`)
