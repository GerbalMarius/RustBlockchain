use externs::{block::Block, lib::now, transaction::{Output, Transaction}};

use crate::externs::blockchain::{self, BlockChain};





mod externs;

fn main() -> Result<(), ()> {
    let difficulty = 0x000fffffffffffffffffffffffffffffu128;//fixed difficulty

    let mut genesis_block = Block::new(0, now(), vec![0;32], vec![
        Transaction{
            inputs : vec![],
            outputs : vec![
                Output{
                    to_addr: "Alice".to_owned(),
                    value : 50,
                },
                Output{
                    to_addr: "Bob".to_owned(),
                    value : 7,
                },
            ],
        },
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block : {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = BlockChain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                Output {
                    to_addr: "Alice".to_owned(),
                    value: 36,
                },
                Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        },
    ], difficulty);

    block.mine();
    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add new transaction block");
    Ok(())
}
