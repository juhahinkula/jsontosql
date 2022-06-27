#![allow(unused)]

use clap::Parser;
use serde_json::{Number, Value};

#[derive(Parser)]
struct Cli {
    table_name: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);

    let data = { 
        serde_json::from_str::<Value>(&content).unwrap() 
    };

    let nb_elements = data.as_array().unwrap().len();

    println!("First country={}", data[0]);

    // Get key & value
    for (key, value) in data[0].as_object().unwrap() {
        println!("{:?} ===> {:?}", key, value);
    }

    Ok(())
}
