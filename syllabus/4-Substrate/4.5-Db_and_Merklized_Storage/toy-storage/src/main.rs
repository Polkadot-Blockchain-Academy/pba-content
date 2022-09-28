//! This project builds a demonstration version of the kind of MerklePatricia-based
//! storage described in the Storage lecture. By building this somewhat simplified
//! of the storage, you will learn

use std::collections::HashMap;

fn main() {
	//  We'll use Strings like Shawn's slides
	let values_to_store = vec!["parity", "participate", "party", "process", "procure", "prospective"];

	// This represents our Key-Value Database. In Substrate this would
	// be RocksDB or ParityDB. The key type is the hash of the Node itself.
	let mut kvdb = ::new();
	let storage = StorageInstance()

	println!("Hello, world!");
}


/// The key type is arbitrary data modelled as a simple bitstring Vec<bool>
/// This choice also allows the use of a binary tree.
/// Nodes need to be hashable so we ca nuse them in the underlying kvdb.
#[derive(Hash)]
pub enum Node<Value> {
	Empty,
	Leaf {
		remaining_key: Vec<bool>,
		value: Value,
	},
	BranchNoValue {
		partial_key: Vec<bool>,
		leftChild: Box<Self>,
		rightChild: Box<Self>,
	},
	BranchWithValue {
		partial_key: Vec<bool>,
		leftChild: Box<Self>,
		rightChild: Box<Self>,
		value: Value,
	},
}

/// Represents a complete storage implemented using the Particia Merkle trie strategy.
/// Underlying KeyValue storage is a simple hashmap.
pub type StorageInstance<Value>{
	kvdb: HashMap::<u64, Node<Value>>,
}

impl StorageInstance<Value> {
	//! Check whether a particular key is in the trie and if it is, return its value.
	pub fn get(key: Vec<bool>) -> Option<Value> {
		todo!("Exercise")
	}

	//! Check whether the 
	pub fn is_empty() -> bool {
		todo!("exercise")
	}

	//! The root node is stored in the kvdb just like every other node.
	//! It is always stored at the same fixed key, namely, the empty vector.
	pub fn root() -> Node<Value> {
		todo!("exercise")
	}

	pub fn insert(key: Vec<bool>, value: Value) {
		todo!("Exercise")
	}
}


// Dumb Question: Why are we building this whole logical trie instead
// of just storing data in the kvdb directly?