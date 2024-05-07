use externs::block::Block;

use crate::externs::{blockchain::{self, BlockChain}, hashable::Hashable, lib::now};



mod externs;

fn main() -> Result<(), ()> {
    let difficulty = 0x000fffffffffffffffffffffffffffffu128;
    let mut block = Block::new(0, now(), vec![0;32], 0, "Genesis block!".to_owned(), 
    difficulty);
    
    block.hash = block.hash();

    println!("Before mining:{:?}", &block);


    block.mine();
    let mut last_hash = block.hash.clone();
    println!("Mined genesis block:{:?}", &block);

    

    let mut blockchain = BlockChain{
        blocks : vec![block]
    };
    println!("Verify of genesis_block :{}", &blockchain.verify());
    

    for i in 1..=10 {

    let mut block = Block::new(i, now(), last_hash, 0, "Another block!".to_owned(), 
    difficulty);
    
    block.hash = block.hash();

    println!("MINE TEST START");
    println!("Before mining:{:?}", &block);


    block.mine();
    println!("Mined  block:{:?}", &block);
    last_hash = block.hash().clone();
    blockchain.blocks.push(block);
    println!("Verify:{}", &blockchain.verify());
    }
    //should fail the integrity check
    blockchain.blocks[3].payload = "Nope".to_owned();
    println!("Verify:{}", &blockchain.verify());
    Ok(())
}
