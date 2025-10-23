#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use clap::{command, Parser};

pub mod problems;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[arg(short, long)]
    id: u16,
}

fn main() {
    let args = App::parse();

    let mut hm: HashMap<u16, fn()> = HashMap::new();
    hm.insert(1, problems::id0001::run);
    hm.insert(2, problems::id0002::run);
    hm.insert(916, problems::id0916::run);
    hm.insert(407, problems::id0407::run);
    hm.insert(1400, problems::id1400::run);
    hm.insert(1408, problems::id1408::run);
    hm.insert(1518, problems::id1518::run);
    hm.insert(1769, problems::id1769::run);
    hm.insert(2116, problems::id2116::run);
    hm.insert(2185, problems::id2185::run);
    hm.insert(2221, problems::id2221::run);
    hm.insert(3042, problems::id3042::run);
    hm.insert(3346, problems::id3346::run);

    match hm.get(&args.id) {
        Some(f) => {
            f();
            // if f() exits successfully, we know all assert_eq! macros have been satisfied and we're clear for submission
            println!("all example cases passed!");
        }
        None => println!("failed to find solution for problem {}", args.id),
    }
}
