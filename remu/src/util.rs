use std::num::ParseIntError;

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }}
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        // 📕: error message
        // 📙: warning message
        // 📗: ok status message
        // 📘: action message
        // 📓: debug status message
        // 📔: Or anything you like and want to recognize immediately by color
        print!("{}", $crate::color::bold(&$crate::color::blue("[INFO]: ")));
        print!("{:<40} [{}:{}:{}] ",
        $crate::function!(), file!(), line!(), column!());
        println!($($arg)*);
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        print!("{}", $crate::color::bold(&$crate::color::yellow("[WARN]: ")));
        print!("{} [{}:{}:{}] ",
        $crate::function!(), file!(), line!(), column!());
        println!($($arg)*);
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        print!("{}", $crate::color::bold(&$crate::color::red("[ERROR]: ")));
        print!("{} [{}:{}:{}] ",
        $crate::function!(), file!(), line!(), column!());
        println!($($arg)*);
    })
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        print!("{}", $crate::color::bold(&$crate::color::green("[DEBUG]: ")));
        print!("{} [{}:{}:{}] ",
        $crate::function!(), file!(), line!(), column!());
        println!($($arg)*);
    })
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => ({
        print!("{}", $crate::color::bold(&$crate::color::red("[FATAL]: ")));
        print!("{} [{}:{}:{}] ",
        $crate::function!(), file!(), line!(), column!());
        println!($($arg)*);
    })
}


pub trait LinearParse {
    fn linearparse(input: &[u8]) -> Self;
}

pub fn parse_str(s: &str) -> Result<u32, ParseIntError> {
    if let Some(new) = s.strip_prefix("0x") {
        u32::from_str_radix(new, 16)
    } else {
        s.parse::<u32>()
    }
}
