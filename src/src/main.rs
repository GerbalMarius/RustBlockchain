use externs::block::Block;

use crate::externs::hashable::Hashable;



mod externs;

fn main() -> Result<(), ()> {
    let mut block = Block::new(0, 0, vec![0;32], 118318, "Genesis block!".to_owned(), 
    0x0000ffffffffffffffffffffffffffff);
    
    block.hash = block.hash();

    println!("{:?}", &block);

    

    Ok(())
}
