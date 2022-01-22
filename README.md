# What 
Very simply "n" argument extension to core::mem::swap.

# Motivation
Wrote it for doing a linked-list question on leetcode

# Usage 
Add the following to your Cargo.toml
```
swap-n = "0.1"
```

Use either `#[macro_use] extern crate swap_n;` or for newer versions, directly call `swap_n::swap_n` or via `use swap_n::swap_n;`

# Example
```rust
use swap_n::swap_n;

fn main() {
    let mut x = std::boxed::Box::new(1);
    let mut y = std::boxed::Box::new(2);
    let mut z = std::boxed::Box::new(3);
    let mut w = std::boxed::Box::new(4);

    println!("The old values of x, y, z, w: {} {} {} {}", *x, *y, *z, *w);

    swap_n!(&mut x, &mut y, &mut z, &mut w);

    println!("The new values of x, y, z, w: {} {} {} {}", *x, *y, *z, *w);
}
```



# License
MIT license