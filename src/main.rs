// Todo
// - Monitor current second in song, if goes back, then re record song as been replayed

mod info;
mod lock;

pub use crate::info::*;
pub use crate::lock::*;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You must pass one argument, and one argument only.");
        process::exit(1);
    }
    let file_location = args[1].to_string();
    println!("{}", file_location);

    if lock::is_locked() {
        println!("All ready running!");
        process::exit(1);
    }

    lock::lock_access();

    let random_string =
        "AEuJXHeUr7sKhwuWntS5wnitC5cTdtx3piRPp2Q5aDxrzqh5vZj4PyhQShJVWaTW".to_string();
    info::get_info(random_string, file_location);
}
