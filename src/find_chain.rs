extern crate serde;
extern crate serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Node {
  pub Word: String,
  pub ID: i32,
  pub Next: Vec<i32>,
}

pub fn read_graph(data: &str) -> Vec<Node> {
  serde_json::from_str(data).unwrap()
}

pub struct MaxLoopCount {
  pub value: i64,
}

pub type IDToNextMap = HashMap<i32, Vec<i32>>;
pub type WordToIDMap = HashMap<String, i32>;

/// make a word->Graph map
pub fn make_id_to_node_map(graph: &[Node]) -> IDToNextMap {
  let mut result: IDToNextMap = HashMap::new();
  for node in graph {
    result.insert(node.ID, node.Next.clone());
  }
  result
}

/// make a word->id map
pub fn make_word_to_id_map(graph: &[Node]) -> WordToIDMap {
  let mut result: WordToIDMap = HashMap::new();
  for node in graph {
    result.insert(node.Word.clone(), node.ID);
  }
  result
}

/// perform a dfs into Graph to find longest idiom chain
pub fn find_longest_chain(
  id: i32,
  chain: &[i32],
  node_map: &IDToNextMap,
  max_loop_count: &mut MaxLoopCount,
) -> Vec<i32> {
  max_loop_count.value -= 1;
  if max_loop_count.value < 0 {
    return chain.to_owned();
  }

  let next_words = node_map.get(&id).unwrap();
  let mut valid_next_words: Vec<i32> = Vec::new();

  for id in next_words {
    if !chain.contains(&id) {
      valid_next_words.push(*id)
    }
  }
  if valid_next_words.is_empty() {
    return chain.to_owned();
  }
  let mut max_length: i32 = -1;
  let mut longest_chain: Vec<i32> = Vec::new();
  for id in valid_next_words {
    let mut added_chain = chain.to_owned();
    added_chain.push(id);
    let current_chain = find_longest_chain(id, &added_chain, node_map, max_loop_count);
    let current_length = current_chain.len() as i32;
    if current_length > max_length {
      max_length = current_length;
      longest_chain = current_chain;
    }
  }
  longest_chain
}

pub fn map_id_to_word(graph: &[Node], id_list: &[i32]) -> Vec<String> {
  let mut id_to_word_map: HashMap<i32, String> = HashMap::new();
  for node in graph {
    id_to_word_map.insert(node.ID, node.Word.clone());
  }

  let mut result: Vec<String> = Vec::new();
  for id in id_list {
    let word: String = id_to_word_map.get(&id).unwrap().to_string();
    result.push(word);
  }
  result
}
