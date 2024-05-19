# cuid2-timeless

An unstable, barely tested, probably giving wrong output and might panic out of nowhere CUID2 implementation in Rust. Built for learning more rust and I don't know how to test it.

## Installation

`cargo add cuid2_timeless` with feature flag either `sha2` or `sha3`

## Usage

Either use the `cuid2_timeless::cuid_wrapper()` and call it again to get ID or get some customization with `cuid2_timeless::Cuid`

``` rust
use cuid2_timeless;

let mut cuid = cuid2_timeless::cuid_wrapper();
println!("{}", cuid().unwrap());
```

or some customization

```rust
use cuid2_timeless;
use rand::{self, Rng};

let mut cuid = cuid2_timeless::Cuid::new(
    Box::new( // A randomizer function
        || {
            rand::thread_rng().gen()
        }
    ),
    Box::new( // Your very own counter function
        |mut i| {
            Box::new(move || {
                i += 1;
                i
            })
        }
    ),
    24, // Length
    Box::new(cuid2_timeless::utils::create_fingerprint) // And a fingerprint creator function (im too lazy to implement)
);
cuid.generate(None).unwrap();
```

or verify if it actually is `CUID`

```rust
use cuid2_timeless;

println!("{}", cuid2_timeless::is_cuid("f9ovgvsnly7h6khwt4nx08kom", None, None));
```
