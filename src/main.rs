#[macro_use]
extern crate serde_derive;

mod find_chain;
use find_chain::*;
mod db;
use db::DATA;

fn main() {
  let graph = read_graph(DATA);

  let word_to_node_map = make_word_to_node_map(&graph);
  let res = find_longest_chain(1, &vec![1], &word_to_node_map, 1000);
  println!("res: {:?}", res);
}
