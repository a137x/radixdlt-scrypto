#[cfg(windows)]
use colored::*;
use radix_clis::error::exit_with_error;
use radix_clis::resim;

pub fn main() {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();
    match resim::run() {
        Err(msg) => exit_with_error(msg, 1),
        _ => {}
    }
}