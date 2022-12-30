mod blockchain;
use blockchain::Chain;
use tiny_gradient::{GradientStr, Gradient};

use std::{io, ops::Index};


fn main() {
  print!("{}[2J", 27 as char);

  let mut block_chain = Chain::new();

  println!("{}", "~~~RustChain~~~".gradient(Gradient::Fruit));
  
  let stdin = io::stdin();

  println!("{}",format!("\nCurrent Blockchain length: {:?}", &block_chain.get_blocks().len()).gradient(Gradient::Atlast));

  loop {
    
    print!("{}", "(/f - finish)\nWrite data to create a new block:\n".gradient(Gradient::Cristal));

    let mut data = String::new();
    stdin.read_line(&mut data).expect("could not read line");

    if data.trim() == "/f" {
      break;
    }

    block_chain.generate_block(data);
  }

  println!("{}", format!("{:?}", &block_chain.get_blocks()).gradient(Gradient::Instagram));
}

