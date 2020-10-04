# Stream #

Stream is a library aiming to implement something similar to the [Java Stream Interface](https://docs.oracle.com/javase/8/docs/api/java/util/stream/Stream.html). A Stream might be viewed similar to an [Iterator](https://doc.rust-lang.org/std/iter/index.html), but with the goal of operating at a higher level than an Iterator.

## Motivation ##

Two parts:

- A toy example to learn a little more about Rust & Making libraries
- Potential value in providing a higher-order interface over iterators

## Alternatives ##

- [Itertools](https://docs.rs/itertools/0.9.0/itertools/) might get you close 
- [pipe-trait](https://github.com/KSXGitHub/pipe-trait) depending on your goals