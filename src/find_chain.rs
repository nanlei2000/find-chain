use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Node {
  pub Word: String,
  pub ID: u16,
  pub Next: Vec<u16>,
}

pub fn read_graph(data: &str) -> Vec<Node> {
  serde_json::from_str(data).unwrap()
}

pub type IDToNextMap<'a> = HashMap<u16, &'a Vec<u16>>;

/// make a word->Graph map
pub fn make_id_to_node_map(graph: &[Node]) -> IDToNextMap {
  let mut result: IDToNextMap = HashMap::new();
  for node in graph {
    result.insert(node.ID, &node.Next);
  }
  result
}

/// perform a dfs into Graph to find longest idiom chain
pub fn find_longest_chain(
  id: u16,
  chain_set: HashSet<u16>,
  chain: Vec<u16>,
  node_map: &IDToNextMap,
  max_loop_count: &mut i64,
) -> Vec<u16> {
  if *max_loop_count < 0 {
    return chain;
  }
  *max_loop_count = *max_loop_count - 1;

  let next_words = *node_map.get(&id).unwrap();
  let mut valid_next_words_v: Vec<&u16> = next_words
    .iter()
    .filter(|id| !chain_set.contains(id))
    .collect();
  if valid_next_words_v.is_empty() {
    return chain;
  }
  let mut max_length: u16 = 0;
  let mut longest_chain: Vec<u16> = Vec::new();

  if *max_loop_count % 1000 == 0 {
    let mut rng = thread_rng();
    valid_next_words_v.shuffle(&mut rng);
  }

  valid_next_words_v.iter().for_each(|id| {
    let mut added_chain_set = chain_set.to_owned();
    added_chain_set.insert(**id);
    let mut added_chain = chain.to_owned();
    added_chain.push(**id);
    let current_chain =
      find_longest_chain(**id, added_chain_set, added_chain, node_map, max_loop_count);
    let current_length = current_chain.len() as u16;
    if current_length > max_length {
      max_length = current_length;
      longest_chain = current_chain;
    }
  });

  longest_chain
}

pub fn map_id_to_word<'graph>(
  graph: &'graph Vec<Node>,
  id_list: &'graph Vec<u16>,
) -> Vec<&'graph str> {
  let mut id_to_word_map: HashMap<u16, &str> = HashMap::new();
  for node in graph {
    id_to_word_map.insert(node.ID, &node.Word);
  }

  let mut result: Vec<&str> = Vec::new();
  for id in id_list {
    let word = match id_to_word_map.get(&id) {
      Some(w) => w,
      _ => continue,
    };
    result.push(word);
  }
  result
}
