use std::{str::FromStr};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
  index: i32,
  hash: String,
  prev_hash: String,
  data: String,
  timestamp: String,
  difficulty: i32,
  nonce: i32
}


pub struct Chain {
  blocks: Vec<Block>
}

impl Chain {
  pub fn new() -> Self {
  let mut blocks = Vec::<Block>::new();
  
  let mut sha256 = Sha256::new();
  sha256.update("genesis block");
  let hash = format!("{:X}", sha256.finalize());

  let timestamp = chrono::offset::Utc::now();

  let genesis_block = Block {
    index: 0,
    hash,
    prev_hash: String::from_str("").unwrap(),
    data: String::new(),
    timestamp: timestamp.to_string(),
    nonce: 0,
    difficulty: 0, 
  };

  blocks.push(genesis_block);
  
  Self {blocks}
  }

  pub fn get_blocks(&self) -> &Vec<Block> {
    &self.blocks
  }

  
}


