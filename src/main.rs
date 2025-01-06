use std::collections::HashMap;

use clap::{command, Parser};

mod problems;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[arg(short, long)]
    id: u16,
}

fn main() {
    let args = App::parse();

    let mut hm = HashMap::<u16, fn()>::new();
    hm.insert(1769, problems::id1769::run);

    match hm.get(&args.id) {
        Some(f) => f(),
        None => todo!(),
    }
}
