const RED  : &str = "\x1b[31m";
const BLUE : &str = "\x1b[34m";
const CLEAR : &str = "\x1b[0m";
const GREEN : &str = "\x1b[32m";
const YELLOW : &str = "\x1b[33m";
pub const ERROR : &str = "\x1b[1m\x1b[31merror\x1b[0m";


pub fn red(s: &str) -> String {
    format!("{}{}{}", RED, s, CLEAR)
}

pub fn blue(s: &str) -> String {
    format!("{}{}{}", BLUE, s, CLEAR)
}

pub fn green(s: &str) -> String {
    format!("{}{}{}", GREEN, s, CLEAR)
}

pub fn yellow(s: &str) -> String {
    format!("{}{}{}", YELLOW, s, CLEAR)
}

pub fn bold(s: &str) -> String {
    format!("{}{}{}", "\x1b[1m", s, CLEAR)
}

pub fn bold_red(s: &str) -> String {
    format!("{}{}{}{}", "\x1b[1m", RED, s, CLEAR)
}

pub fn bold_green(s: &str) -> String {
    format!("{}{}{}{}", "\x1b[1m", "\x1b[32m", s, CLEAR)
}

pub fn grey(s: &str) -> String {
    format!("{}{}{}", "\x1b[1;30m", s, CLEAR)
}