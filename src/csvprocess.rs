use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
  name: String,
  age: u8,
  gender: String,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
  let mut reader = Reader::from_path(input)?;
  let mut ret: Vec<Person> = Vec::with_capacity(128);
  for result in reader.deserialize() {
    let record: Person = result?;
    ret.push(record);
  }
  let json = serde_json::to_string_pretty(&ret)?;
  fs::write(output, json)?;
  Ok(())
}
