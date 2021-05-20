# rust-kata

![example workflow](https://github.com/krscott/rust-kata/actions/workflows/rust.yml/badge.svg)

CodeKata practice using Rust

http://codekata.com/


## Kata02 - Karate Chop
Each day for five days, write a unique implementations of a binary search
according to the [specification](http://codekata.com/kata/kata02-karate-chop/).
Assume the array is a sorted list of unique integers.

I decided to use `None` instead of `-1` to represent index-not-found so I could
return `Option<usize>` instead of needing to convert the index to `isize`. This
is also more idiomatic Rust.

Days:
1. `chop1()` - Imperative.
   Typical implementation--no issues.
2. `chop2()` - Functional.
   I learned that arrays can be pivoted with Python-like syntax.

   First run: overflow error from wrong ranging.

   Second run: Found I forgot to add pivot to output of recursive call, so function
   would always return an index <= half-length. Easy fix.

3. *TODO*
4. *TODO*
5. *TODO*

