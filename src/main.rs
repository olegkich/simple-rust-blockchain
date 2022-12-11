mod blockchain;

use blockchain::Chain;


fn main() {
  let block_chain = Chain::new();
  
  let blocks = block_chain.get_blocks();

  println!("~~~RustChain~~~");

  // todo - input block to add to blockchain
  loop {

  }
}

