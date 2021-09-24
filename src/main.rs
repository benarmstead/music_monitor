// Todo
// - Monitor current second in song, if goes back, then re record song as been replayed

mod analysis;
mod info;
mod lock;

pub use crate::info::*;
pub use crate::lock::*;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len > 3 || args_len == 1 {
        println!("You must pass 1 or 2 arguments only.");
        process::exit(1);
    }
    let file_location = args[1].to_string();
    println!("{}", file_location);

    if args_len == 2 {
        start_monitor(file_location);
    } else if args_len == 3 && args[2] == "-d" {
        start_analyse(file_location);
    }
}

fn start_analyse(file_location: String) {
    analysis::main::start(file_location);
}

fn start_monitor(file_location: String) {
    if lock::is_locked() {
        println!("All ready running!");
        process::exit(1);
    }

    lock::lock_access();

    let random_string =
        "AEuJXHeUr7sKhwuWntS5wnitC5cTdtx3piRPp2Q5aDxrzqh5vZj4PyhQShJVWaTW".to_string();
    info::get_info(random_string, file_location);
}
