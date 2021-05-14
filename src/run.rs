use crate::db::DATA;
use crate::find_chain::*;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run(word: &str, mut max_loop_count: i64) {
  println!(
    "[info] head idiom: {}, max loop count: {}",
    word, max_loop_count
  );
  let graph: Vec<Node> = read_graph(DATA);
  let mut id: Option<u16> = None;
  for item in graph.iter() {
    if item.word == *word {
      id = Some(item.id);
      break;
    }
  }
  assert!(
    id.is_some(),
    "[ERROR] can't find word \"{}\" in database",
    word
  );
  let id = id.unwrap();
  let id_to_next_map = make_id_to_node_map(&graph);
  let mut set: HashSet<u16> = HashSet::new();
  set.insert(id);

  let now = get_unix_now();
  let chain: Vec<u16> = find_longest_chain(id, set, vec![id], &id_to_next_map, &mut max_loop_count);
  let duration = get_unix_now() - now;

  let words = map_id_to_word(&graph, &chain);
  println!("[info] chain: {:?}", words);
  println!("[info] length: {}", chain.len());
  println!("[info] dfs took: {}ms", duration);
}

fn get_unix_now() -> i64 {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  since_the_epoch.as_secs() as i64 * 1000i64
    + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64
}

#[test]
fn test_run() {
  run("为所欲为", 10_000)
}
