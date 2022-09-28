//! This project builds a demonstration version of the kind of MerklePatricia-based
//! storage described in the Storage lecture. By building this somewhat simplified
//! of the storage, you will learn

use std::collections::HashMap;

fn main() {
	//  We'll use Strings like Shawn's slides
	let values_to_store = vec!["parity", "participate", "party", "process", "procure", "prospective"];

	// This represents our Key-Value Database. In Substrate this would
	// be RocksDB or ParityDB.
	let mut kvdb = HashMap::new();

	println!("Hello, world!");
}

/// The key type is arbitrary data modelled as a simple bitstring Vec<bool>
/// This choice also allows the use of a binary tree.
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