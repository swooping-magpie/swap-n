#![no_std]

//! Macro to extend core::mem::swap to n elements
//!
//! Example usage:
//!
//! ```rust
//! // #[macro_use] extern crate swap_n;
//! use swap_n::swap_n;
//!     
//! let mut x = 1;
//! let mut y = 2;
//! let mut z = 3;
//!
//! swap_n!(&mut x, &mut y, &mut z);
//! ```
//!
//!
//!
//! Examples of failure:
//!
//! ```compile_fail
//! use swap_n::swap_n;
//!
//! let mut x = 1;
//! swap_n!(&mut x);
//! ```
//!
//! ```compile_fail
//! use swap_n::swap_n;
//!
//! let mut x = 1;
//! swap_n!(x);
//! ```
//!
//!
//!
//! ```compile_fail
//! use swap_n::swap_n;
//!
//! let mut x = 1;
//! let mut y = 1.0;
//! swap_n!(&mut x, &mut y);
//! ```
//!
//!
//!

#[macro_export]
macro_rules! swap_n{
    ($first:expr, $second:expr, $($e:expr),+) => {
            core::mem::swap($first, $second);
            let mut _prev = $second;
            $(
                core::mem::swap(_prev, $e);
                _prev = $e;
            )+
            //core::mem::swap(_prev, $first);
    };
    ($first:expr, $second:expr) => {
        core::mem::swap($first, $second);
    };
    ($first:expr) => {
        compile_error!("This macro only makes sense with strictly greater than 1 arguments!");
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn most_basic_functionality() {
        let mut x = 1;
        let mut y = 2;
        let mut z = 3;

        swap_n!(&mut x, &mut y);

        assert_eq!(x, 2);
        assert_eq!(y, 1);

        swap_n!(&mut x, &mut y);

        assert_eq!(x, 1);
        assert_eq!(y, 2);

        swap_n!(&mut x, &mut y, &mut z);
        assert_eq!(x, 2);
        assert_eq!(y, 3);
        assert_eq!(z, 1);
        //swap_n!(&mut x);
    }

    #[test]
    fn four_elements() {
        let mut x = 1;
        let mut y = 2;
        let mut z = 3;
        let mut w = 4;

        swap_n!(&mut x, &mut y, &mut z, &mut w);
        assert_eq!(x, 2);
        assert_eq!(y, 3);
        assert_eq!(z, 4);
        assert_eq!(w, 1);
    }
}
