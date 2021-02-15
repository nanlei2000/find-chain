extern crate serde;
extern crate serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Node {
  Word: String,
  ID: i32,
  Next: Vec<i32>,
}

pub fn read_graph(data: &str) -> Vec<Node> {
  let v: Vec<Node> = serde_json::from_str(data).unwrap();
  return v;
}

type IDToNodeMap = HashMap<i32, Node>;

// MakeWordToGraphItemMap make a word->Graph map
pub fn make_word_to_node_map(graph: &Vec<Node>) -> IDToNodeMap {
  let mut result: IDToNodeMap = HashMap::new();
  for node in graph {
    result.insert(node.ID, node.clone());
  }
  return result;
}

/// FindLongestChain perform a dfs into Graph to find longest idiom chain
pub fn find_longest_chain(
  id: i32,
  chain: &Vec<i32>,
  node_map: &IDToNodeMap,
  max_loop_count: i64,
) -> Vec<i32> {
  println!("count: {}", max_loop_count);
  // max_loop_count = max_loop_count - 1;
  if max_loop_count < 0 {
    println!("here");
    return chain.clone();
  }

  let next_nodes: Node = node_map.get(&id).unwrap().clone();
  let mut valid_next_words: Vec<i32> = Vec::new();

  for id in next_nodes.Next {
    if !chain.contains(&id) {
      valid_next_words.push(id)
    }
  }
  if valid_next_words.len() == 0 {
    return chain.clone();
  }
  let mut max_length: i32 = -1;
  let mut longest_chain: Vec<i32> = Vec::new();
  for id in valid_next_words {
    let mut added_chain = chain.clone();
    added_chain.push(id);
    let current_chain = find_longest_chain(id, &added_chain, node_map, max_loop_count - 1);
    let current_length = current_chain.len() as i32;
    println!("current_length: {}", current_length);
    if current_length > max_length {
      max_length = current_length;
      longest_chain = current_chain;
    }
  }

  return longest_chain;
}
