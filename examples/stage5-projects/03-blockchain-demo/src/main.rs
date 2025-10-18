use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut headers = self.index.to_string();
        headers.push_str(&self.timestamp.to_string());
        headers.push_str(&self.data);
        headers.push_str(&self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(headers.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain { blocks: vec![genesis_block] }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        self.blocks.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First block after genesis".to_string());
    blockchain.add_block("Second block after genesis".to_string());

    for block in blockchain.blocks {
        println!("{:#?}", block);
    }
}
