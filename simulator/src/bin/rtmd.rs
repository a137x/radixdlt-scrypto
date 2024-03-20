#[cfg(windows)]
use colored::*;
use simulator::error::exit_with_error;
use simulator::rtmd;

pub fn main() {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();
    match rtmd::run() {
        Err(msg) => exit_with_error(msg, 1),
        _ => {}
    }
}
