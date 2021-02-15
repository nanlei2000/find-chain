#[macro_use]
extern crate serde_derive;

mod find_chain;
use find_chain::*;
mod db;
use db::DATA;

fn main() {
  let graph = read_graph(DATA);
  let word_to_id_map: WordToIDMap = make_word_to_id_map(&graph);
  let id: i32 = word_to_id_map.get("文武双全").unwrap().clone();
  let id_to_next_map = make_id_to_node_map(&graph);
  let res: Vec<i32> = find_longest_chain(
    id,
    &vec![id],
    &id_to_next_map,
    &mut MaxLoopCount { value: 100_000_000 },
  );
  let words = map_id_to_word(&graph, &res);
  println!("words: {:?}", words);
  println!("length: {}", words.len());
}
