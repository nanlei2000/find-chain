#[macro_use]
extern crate serde_derive;
mod db;
mod find_chain;
use std::env;
mod run;

fn main() {
  let args: Vec<String> = env::args().collect();
  let max_loop_count = args[1].parse::<i64>().unwrap();
  let word = &args[2];

  crate::run::run(word, max_loop_count)
}
