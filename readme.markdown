# gcd

Compute the greatest common divisor using
[Euclid's algorithm](https://en.wikipedia.org/wiki/Greatest_common_divisor#Using_Euclid.27s_algorithm).

[![build status](https://api.travis-ci.org/tomjakubowski/rust-gcd.svg)](http://travis-ci.org/tomjakubowski/rust-gcd)

# example

```rust
extern crate gcd;
use gcd = gcd::gcd;

pub fn main() {
    println!("{}", gcd(121i64, 44));
}
```

***

```
11
```

# license

WTFPL
