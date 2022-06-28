use std::error::Error;
use clap::Parser;
use serde_json::{Number, Value};

#[derive(Parser)]
struct Cli {
    table_name: String,
    #[clap(parse(from_os_str))]
    file: std::path::PathBuf,
}

pub fn run() ->  Result<(), Box<dyn Error>> {
  let args = Cli::parse();

  let result = std::fs::read_to_string(&args.file);
  let content = match result {
      Ok(content) => { content },
      Err(error) => { return Err(error.into()); }
  };

  let data = { 
    serde_json::from_str::<Value>(&content).unwrap() 
  };

  // Get key & value
  for (key, value) in data[0].as_object().unwrap() {
      println!("INSERT INTO {} VALUES ({:?} ===> {:?})", &args.table_name, key, value);
  }

  Ok(())  
}