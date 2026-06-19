use crossbeam_queue::SegQueue;
use std::sync::Arc;
use pqcrypto_dilithium::dilithium2::{
    PublicKey, DetachedSignature, verify_detached_signature
};

pub struct Transaction {
    pub id: u64,
    pub public_key: PublicKey,
    pub signature: DetachedSignature,
    pub message: Vec<u8>,
    pub is_valid: bool,
}

impl Transaction {
    pub fn new(id: u64, pk: PublicKey, sig: DetachedSignature, msg: Vec<u8>) -> Self {
        Self { 
            id, 
            public_key: pk,
            signature: sig,
            message: msg,
            is_valid: false,
        }
    }

    pub fn validate(&mut self) {
        // Usamos os tipos concretos diretamente aqui
        match verify_detached_signature(&self.signature, &self.message, &self.public_key) {
            Ok(_) => self.is_valid = true,
            Err(_) => self.is_valid = false,
        }
    }
}

pub struct Mempool {
    queue: Arc<SegQueue<Transaction>>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { queue: Arc::new(SegQueue::new()) }
    }

    pub fn push(&self, tx: Transaction) {
        self.queue.push(tx);
    }

    pub fn select_for_block(&self, limit: usize) -> Vec<Transaction> {
        let mut block = Vec::with_capacity(limit);
        while block.len() < limit {
            match self.queue.pop() {
                Some(tx) => block.push(tx),
                None => break,
            }
        }
        block
    }
}
