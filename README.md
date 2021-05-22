# rust-kata

![example workflow](https://github.com/krscott/rust-kata/actions/workflows/rust.yml/badge.svg)

CodeKata practice using Rust

http://codekata.com/


## Kata02 - Karate Chop
Each day for five days, write a unique implementations of a binary search
according to the [specification].
Assume the array is a sorted list of unique integers.

I decided to use `None` instead of `-1` to represent index-not-found so I could
return `Option<usize>` instead of needing to convert the index to `isize`. This
is also more idiomatic Rust. Converting back to original spec is trivial.

Days:
1. `chop1()` - Imperative.
   Typical implementation. No issues.

2. `chop2()` - Functional.
   Functional-style implementation which operates on array sub-slices.

   I learned that arrays can be pivoted with Python-like syntax.

   First run: overflow error from wrong ranging.

   Second run: Found I forgot to add pivot to output of recursive call, so function
   would always return an index <= half-length. Easy fix.

3. `chop3()` - Recursive Imperative.
   Recursive implementation which narrows down an index range at each level.

   This was an awkward implementation for me. And so, several times I hit "attempt to subtract with
   overflow" error. I'm glad Rust does this error checking. In C this would have overflowed
   silently and would have been harder to track down. Clearly, Rust is trying to nudge me away from
   this kind of implementation.

4. `chop4()` - Imperative Mutable slice.
   An imperative implementation of `chop2`. No issues.

5. `chop5()` - Declarative.
   An implementation using Rust iterators.

   I looked up the [slice docs] and found `chunks()` and tried to find a way to use it. It worked
   out well, though it is a little hard to read.


[specification]: http://codekata.com/kata/kata02-karate-chop/
[slice docs]: https://doc.rust-lang.org/std/primitive.slice.html
