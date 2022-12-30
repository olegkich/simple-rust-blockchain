use std::{str::FromStr};
use chrono::{DateTime, Utc, Timelike};
use sha2::{Sha256, Digest};
use tiny_gradient::{Gradient, GradientStr, RGB};
#[derive(Debug)]
pub struct Block {
  index: usize,
  hash: String,
  previous_hash: String,
  data: String,
  timestamp: DateTime<Utc>,
  difficulty: usize,
  nonce: usize
}

// seconds
const BLOCK_GENERATION_INTERVAL: usize = 10;
// blocks
const DIFFICULTY_ADJUSTMENT_INTERVAL: usize  = 5;

pub struct Chain {
  blocks: Vec<Block>,
  difficulty: usize
}

impl Chain {
  pub fn new() -> Self {
    let blocks = Self::create_blockchain();
    Self {blocks, difficulty: 1}
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
      timestamp,
      nonce: 0,
      difficulty: 1, 
    };

    blocks.push(genesis_block);

    blocks
  }

  pub fn get_blocks(&self) -> &Vec<Block> {
    &self.blocks
  }

  fn get_last_block(&self) -> &Block {
    &self.blocks[self.blocks.len() - 1]
  }

  pub fn generate_block(&mut self, data: String) {
    let chain = &self.blocks;

    let index = chain.len();
    let timestamp = chrono::offset::Utc::now();
    let previous_hash = &self.blocks.get(index - 1).unwrap().hash;
  
    let new_block = self.find_block(data, timestamp, index, previous_hash.to_string() );

    self.blocks.push(new_block);
  }

  fn generate_hash(&self, data: &String, timestamp: &DateTime<Utc>, previous_hash: &String, index: usize, nonce: usize) -> String {
    let mut sha256 = Sha256::new();
    let hash_string: String = format!("{}{}{}{}{}", data, timestamp, previous_hash, index, nonce);
    sha256.update(hash_string);
    let hash = format!("{:X}", sha256.finalize());
    hash
  }


  fn find_block(&self, data: String, timestamp: DateTime<Utc>, index: usize, previous_hash: String) -> Block{
    let mut nonce = 0;

    loop {
      print!("Proof of work: {}\n", nonce);

      let hash = self.generate_hash(&data, &timestamp, &previous_hash, index, nonce);

      if Self::hash_matches_difficulty(&hash, self.get_difficulty(&self.get_blocks())) {
        let block = Block {data, timestamp, previous_hash, index, hash, nonce, difficulty: self.get_difficulty(self.get_blocks())};

        let time_taken = block.timestamp.num_seconds_from_midnight() - self.get_last_block().timestamp.num_seconds_from_midnight();
        let message = format!("generated block in {} seconds with difficulty: {}", time_taken, &block.difficulty.to_string());
        let colored = message.gradient([RGB::new(0x01, 0x00, 0x00), RGB::new(0xDA, 0x00, 0xFF)]);
        println!("{}", colored);

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

  fn get_difficulty(&self, blockchain: &Vec<Block>) -> usize {
    let last_block = self.get_last_block();

    if last_block.index % 5 == 0 && last_block.index != 0 {
      return self.get_adjusted_difficulty(last_block, blockchain);
    } else {
      return last_block.difficulty;
    }
  }

  fn get_adjusted_difficulty(&self, last_block: &Block, blockchain: &Vec<Block>) -> usize {
    let prev_adjusted_block = &blockchain[&blockchain.len() - 5];
    let time_expected = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL;
    let time_taken = last_block.timestamp - prev_adjusted_block.timestamp;
    
    if (time_taken.num_seconds() as usize) < time_expected / 2 {
      return prev_adjusted_block.difficulty.saturating_add(1);
    } else if time_taken.num_seconds() as usize > time_expected * 2 {
      return prev_adjusted_block.difficulty.saturating_sub(1);
    } else {
      return prev_adjusted_block.difficulty;
    }
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
        _ => ""
    }
  }
  
}


