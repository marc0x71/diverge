#![allow(unused_macros)]

#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($($x:tt)*) => { println!($($x)*) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($($x:tt)*) => {{}};
}
