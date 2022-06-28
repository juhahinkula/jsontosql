#![allow(unused)]

use std::process;

fn main() {
    if let Err(e) = jsontosql::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
