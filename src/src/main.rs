use externs::block::Block;

use crate::externs::hashable::Hashable;



mod externs;

fn main() -> Result<(), ()> {
    let mut block = Block::new(0, 0, vec![0;32], 0, "Genesis block!".to_owned());
    println!("{:?}", &block);

    let hash = block.hash();

    println!("{:?}", &hash);

    block.hash = hash;

    println!("{:?}" , &block);

    Ok(())
}
