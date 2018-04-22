//! Blocks routines.

use block::Block;

/// Creates a genesis block and appends it to the chain.
///
/// Args:
///
/// `chain` - the chain to modify
/// `data` - the data to add
pub fn add_genesis_block(chain: &mut Vec<Block>, data: i32) {

    let genesis = Block::new(data, String::new());

    println!("Genesis block has been generated.");
    println!("Current block digest: {}", genesis.get_current());

    chain.push(genesis);
}
