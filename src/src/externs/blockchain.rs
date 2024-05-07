use super::{block::{self, Block}, hashable::Hashable};


pub struct BlockChain{
    pub blocks : Vec<Block>
}

impl BlockChain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch : {} != {}", &block.index, &i);

                return false;

            }else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty check failed.");

                return false;
            }else if i != 0 {
                //Not genesis block 
                let previous_block = &self.blocks[i - 1];
                if block.timestamp <= previous_block.timestamp {
                    println!("Time did not increase");
                    return false;
                }else if block.prev_block_hash != previous_block.hash {
                    println!("Hash mismatch");
                    return false;
                }
            }else {
                //Genesis block
                if block.prev_block_hash != vec![0;32] {
                    println!("Genesis block prev hash block invalid");
                    return false;
                }
            }
        }
        true
    }
}