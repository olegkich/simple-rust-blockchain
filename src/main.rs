mod blockchain;
use blockchain::Chain;

use std::io;


fn main() {
  print!("{}[2J", 27 as char);

  let mut block_chain = Chain::new();

  println!("~~~RustChain~~~");
  
  let stdin = io::stdin();
  println!("\nCurrent Blockchain:\n{:?}\n\n", &block_chain.get_blocks());

  loop {
    
    print!("(/f - finish)\nWrite data to create a new block:\n");

    let mut data = String::new();
    stdin.read_line(&mut data).expect("could not read line");

    if data.trim() == "/f" {
      break;
    }

    block_chain.generate_block(data);
  }

  println!("{:?}", &block_chain.get_blocks());
}

