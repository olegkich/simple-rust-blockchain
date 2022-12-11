use std::{str::FromStr};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
  index: usize,
  hash: String,
  previous_hash: String,
  data: String,
  timestamp: String,
  difficulty: usize,
  nonce: usize
}


pub struct Chain {
  blocks: Vec<Block>,
  difficulty: usize
}

impl Chain {
  pub fn new() -> Self {
    let blocks = Self::create_blockchain();
    Self {blocks, difficulty: 15}
  }

  fn create_blockchain() -> Vec::<Block> {
    let mut blocks = Vec::<Block>::new();
  
    let mut sha256 = Sha256::new();
    sha256.update("genesis block");
    let hash = format!("{:X}", sha256.finalize());

    let timestamp = chrono::offset::Utc::now();

    let genesis_block = Block {
      index: 0,
      hash,
      previous_hash: String::from_str("").unwrap(),
      data: String::new(),
      timestamp: timestamp.to_string(),
      nonce: 0,
      difficulty: 1, 
    };

    blocks.push(genesis_block);

    blocks
  }

  pub fn get_blocks(&self) -> &Vec<Block> {
    &self.blocks
  }

  pub fn generate_block(&mut self, data: String) {
    let chain = &self.blocks;

    let index = chain.len();
    let timestamp = chrono::offset::Utc::now().to_string();
    let previous_hash = &self.blocks.get(index - 1).unwrap().hash;
  
    let new_block = self.find_block(data, timestamp, index, previous_hash.to_string() );

    self.blocks.push(new_block);
  }

  fn generate_hash(&self, data: &String, timestamp: &String, previous_hash: &String, index: usize, nonce: usize) -> String {
    let mut sha256 = Sha256::new();
    let hash_string: String = format!("{}{}{}{}{}", data, timestamp, previous_hash, index, nonce);
    sha256.update(hash_string);
    let hash = format!("{:X}", sha256.finalize());
    hash
  }

  fn find_block(&self, data: String, timestamp: String, index: usize, previous_hash: String) -> Block{
    let mut nonce = 0;

    loop {
      print!("Proof of work: {}\n", nonce);

      let hash = self.generate_hash(&data, &timestamp, &previous_hash, index, nonce);

      if Self::hash_matches_difficulty(&hash, self.difficulty) {
        let block = Block {data, timestamp, previous_hash, index, hash, nonce, difficulty: self.difficulty};
        return block;
      }

      nonce += 1;
    }
  }

  fn hash_matches_difficulty( hash: &String, difficulty: usize) -> bool {
    
    let binary_hash = Self::hex_to_bin(hash);
    let prefix = "0".repeat(difficulty);
    binary_hash.starts_with(&prefix)
  }

  pub fn hex_to_bin(hex: &str) -> String {
    hex[2..].chars().map(Self::to_binary).collect()
  }

  pub fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
  }
  
}


