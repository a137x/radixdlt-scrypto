#[cfg(windows)]
use colored::*;
use simulator::error::exit_with_error;
use simulator::resim;

pub fn main() {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();
    match resim::run() {
        Err(msg) => exit_with_error(msg, 1),
        _ => {}
    }
}
