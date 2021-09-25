mod analysis;
mod info;
mod lock;

pub use crate::info::*;
pub use crate::lock::*;

use rand::{distributions::Alphanumeric, Rng};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len > 4 || args_len == 1 {
        println!("You must pass 1 or 3 arguments only.");
        process::exit(1);
    }
    let file_location = args[1].to_string();
    println!("{}", file_location);

    if args_len == 2 {
        start_monitor(file_location);
    } else if args_len == 4 && args[2] == "-d" {
        analysis::main::start(file_location, args[3].to_string());
    } else {
        println!("Incorrect number of arguments supplied!");
    }
}

fn start_monitor(file_location: String) {
    if lock::is_locked() {
        println!("All ready running!");
        process::exit(1);
    }

    lock::lock_access();

    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    info::get_info(random_string, file_location);
}
