

use super::lib::BlockHash;

#[derive(Debug)]
pub struct Block{
    pub index : u32,
    pub timestamp : u128,
    pub hash : BlockHash,
    pub prev_block_hash : BlockHash,
    pub nonce : u64,
    pub payload : String
}

impl Block {
    //constructs a new block instance
    pub fn new( index : u32, timestamp : u128, hash : BlockHash, prev_block_hash : BlockHash, nonce : u64, payload : String) ->Self {
        Self { 
            index,
             timestamp, 
             hash, 
             prev_block_hash, 
             nonce, 
             payload }
    }
}