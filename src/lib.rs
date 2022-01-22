#![no_std]

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
}
