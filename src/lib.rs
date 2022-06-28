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
  let mut columns = String::new();
  let mut values = String::new();
  for (key, value) in data[0].as_object().unwrap() {
      columns = columns + ", " + &key; 
      values = values + ", " + &value.to_string(); 
  }
  println!("{}", values);
  println!("{}", columns);

  let statement = format!("INSERT INTO {} ({}) VALUES ({});", &args.table_name, columns, values);
  println!("{}", statement);

  Ok(())  
}