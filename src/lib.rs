use std::error::Error;
use std::fs::File;
use std::io::{Write};
use clap::Parser;
use serde_json::{Number, Value};

#[derive(Parser)]
#[clap(author="Juha Hinkula", version="1.0", about="Convert JSON to SQL insert statements")]
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

    let json_data = { 
        serde_json::from_str::<Value>(&content).unwrap() 
    };

    let mut output = File::create("result.sql")?;

    for i in 0..json_data.as_array().unwrap().len()-1 {
        let mut columns = String::new();
        let mut values = String::new();
        let mut is_first = true;
  
        for (key, value) in json_data[i].as_object().unwrap() {
            if is_first {
                columns = columns + &key.trim();
                values = values + &value.to_string().trim();
                is_first = false;
            }
            else {      
                columns = columns + ", " +  &key.trim(); 
                values = values + ", " + &value.to_string().trim(); 
            }
        }
      
        let statement = format!("INSERT INTO {} ({}) VALUES ({});", &args.table_name, &columns.trim(), &values.trim());
        write!(output, "{}\n", &statement)?;
        println!("{}", statement);
    }
    
    Ok(())  
}