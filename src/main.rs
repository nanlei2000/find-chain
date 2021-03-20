#[macro_use]
extern crate serde_derive;
mod find_chain;
use find_chain::*;
mod db;
use db::DATA;
use std::collections::HashSet;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
  let args: Vec<String> = env::args().collect();
  // if args.len() != 3 {
  //   panic!("use like this: cmd 10000 文武双全")
  // }
  let mut max_loop_count = args[1].parse::<i64>().unwrap().to_owned();
  let word = &args[2];
  println!(
    "[info] head idiom: {}, max loop count: {}",
    word, max_loop_count
  );

  // 借用由此开始
  let graph: Vec<Node> = read_graph(DATA);
  let mut id: Option<u16> = None;
  for item in graph.iter() {
    if item.Word == *word {
      id = Some(item.ID);
      break;
    }
  }
  assert!(id.is_some());
  let id = id.unwrap();

  let id_to_next_map = make_id_to_node_map(&graph);

  {
    let now = timestamp();
    let mut set: HashSet<u16> = HashSet::new();
    set.insert(id);
    let res: Vec<u16> = find_longest_chain(id, set, vec![id], &id_to_next_map, &mut max_loop_count);
    let duration = timestamp() - now;
    let words = map_id_to_word(&graph, &res);
    println!("chain: {:?}", words);
    println!("length: {}", res.len());
    println!("hash set dfs took: {}ms", duration);
  }
}

fn timestamp() -> i64 {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  since_the_epoch.as_secs() as i64 * 1000i64
    + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64
}
