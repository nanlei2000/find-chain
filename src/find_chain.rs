extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use std::sync::mpsc::channel;
extern crate scoped_threadpool;
use rayon::prelude::*;
use scoped_threadpool::Pool;

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
#[allow(dead_code)]
/// perform a dfs into Graph to find longest idiom chain
pub fn find_longest_chain_rayon(
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
  let par_iter = next_words.par_iter().filter(|id| !chain.contains(&id));

  let valid_next_words: Vec<_> = par_iter.collect();

  if valid_next_words.is_empty() {
    return chain.to_owned();
  }

  let mut max_length: i32 = -1;
  let mut longest_chain: Vec<i32> = Vec::new();
  for id in valid_next_words {
    let mut added_chain = chain.to_owned();
    added_chain.push(*id);
    let current_chain = find_longest_chain_rayon(*id, &added_chain, node_map, max_loop_count);
    let current_length = current_chain.len() as i32;
    if current_length > max_length {
      max_length = current_length;
      longest_chain = current_chain;
    }
  }
  longest_chain
}

#[allow(dead_code)]
/// perform a dfs into Graph to find longest idiom chain
pub fn find_longest_chain_concurrently(
  id: i32,
  chain: &Vec<i32>,
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

  let (sender, receiver) = channel();

  let mut pool = Pool::new(4);

  for id in valid_next_words {
    pool.scoped(|scoped| {
      let tx = sender.clone();
      let mut added_chain = chain.to_owned();
      added_chain.push(id);
      let count = &mut *max_loop_count;
      scoped.execute(move || {
        let current_chain = find_longest_chain(id, added_chain, node_map, count);
        tx.send(current_chain)
          .expect("channel will be there waiting for the pool");
      });
    });
  }
  for chain in receiver.iter() {
    let len = chain.len() as i32;
    if len > max_length {
      max_length = len;
      longest_chain = chain;
    }
  }
  drop(pool);
  longest_chain
}

/// perform a dfs into Graph to find longest idiom chain
pub fn find_longest_chain(
  id: i32,
  chain: Vec<i32>,
  node_map: &IDToNextMap,
  max_loop_count: &mut MaxLoopCount,
) -> Vec<i32> {
  max_loop_count.value -= 1;
  if max_loop_count.value < 0 {
    return chain;
  }

  let next_words = node_map.get(&id).unwrap();
  let valid_next_words = next_words
    .iter()
    .filter(|id| !chain.iter().any(|prev_id| prev_id == *id));

  if valid_next_words.to_owned().take(1).next().is_none() {
    return chain;
  }

  let mut max_length: i32 = -1;
  let mut longest_chain: Vec<i32> = Vec::new();

  valid_next_words.for_each(|id| {
    let mut added_chain = chain.to_owned();
    added_chain.push(*id);
    let current_chain = find_longest_chain(*id, added_chain, node_map, max_loop_count);
    let current_length = current_chain.len() as i32;
    if current_length > max_length {
      max_length = current_length;
      longest_chain = current_chain;
    }
  });

  longest_chain
}

#[allow(dead_code)]
/// perform a dfs into Graph to find longest idiom chain
pub async fn find_longest_chain_async(
  id: i32,
  chain: Vec<i32>,
  node_map: &IDToNextMap,
  max_loop_count: &mut MaxLoopCount,
) -> Vec<i32> {
  max_loop_count.value -= 1;
  if max_loop_count.value < 0 {
    return chain;
  }

  let next_words = node_map.get(&id).unwrap();
  let mut valid_next_words: Vec<i32> = Vec::new();

  for id in next_words {
    if !chain.contains(&id) {
      valid_next_words.push(*id)
    }
  }
  if valid_next_words.is_empty() {
    return chain;
  }
  let mut max_length: i32 = -1;
  let mut longest_chain: Vec<i32> = Vec::new();
  for id in valid_next_words {
    let mut added_chain = chain.to_owned();
    added_chain.push(id);
    let current_chain = find_longest_chain(id, added_chain, node_map, max_loop_count);
    let current_length = current_chain.len() as i32;
    if current_length > max_length {
      max_length = current_length;
      longest_chain = current_chain;
    }
  }
  longest_chain
}

#[allow(dead_code)]
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

#[test]
pub fn xxxx() {
  // Create a threadpool holding 4 threads
  let mut pool = Pool::new(4);

  let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];

  // Use the threads as scoped threads that can
  // reference anything outside this closure
  pool.scoped(|scoped| {
    // Create references to each element in the vector ...
    for e in &mut vec {
      // ... and add 1 to it in a seperate thread
      scoped.execute(move || {
        *e += 1;
      });
    }
  });

  assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
