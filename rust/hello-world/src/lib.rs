use std::convert::identity;
use std::io::Take;
use std::time;
use std::time::Duration;

// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}
