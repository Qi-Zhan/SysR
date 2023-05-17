#![no_std]
#![no_main]

pub mod cte;
pub mod io;
pub mod klib;
pub mod mpe;
pub mod tm;
pub mod vme;

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    // implement the macro ourselves
    ($($arg:tt)*) => {
        match format_args!($($arg)*) {
            args => {
                use core::fmt::Write;
                let mut sp = $crate::io::SerialPort;
                write!(sp, "{}", args).unwrap();
            }
        }
    };
}
