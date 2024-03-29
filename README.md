# JsonToSql
Rust cli that converts json file into SQL Insert statements

## Prerequisites
Rust

## Installation
Clone the repository and build the project
```
cargo build
```
The command creates an executable file `target/debug/jsontosql`.

## Usage
```
jsontosql <TABLE_NAME> <FILE>
```
,where TABLE_NAME is database table name and FILE is json file. Both arguments are required.

Json file format should be the following
```
[
  {
    "key1": "value1",
    "key2": "value2",
    ...
  },
  {
    "key1": "value1",
    "key2": "value2",
    ...   
  }
  ...
]
```

## Example
You can find countries.json example file from the repository
```
[
  {
    "country": "Afghanistan",
    "abbreviation": "AF"
  },
  {
    "country": "Albania",
    "abbreviation": "AL"
  },
  {
    "country": "Algeria",
    "abbreviation": "DZ"
  },
  ...
]
```
Using the command
```
jsontosql country .\countries.json
```
It creates result.sql file that looks the following
```
INSERT INTO country (abbreviation, country) VALUES ("AF", "Afghanistan");
INSERT INTO country (abbreviation, country) VALUES ("AL", "Albania");
INSERT INTO country (abbreviation, country) VALUES ("DZ", "Algeria");
...
```
