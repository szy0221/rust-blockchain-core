mod block;
mod chain;

use chain::Blockchain;

fn main() {
    let mut chain = Blockchain::new();
    chain.add_block("First transaction batch".into());
    chain.add_block("Second transaction batch".into());

    println!("Blockchain length: {}", chain.blocks.len());
}
