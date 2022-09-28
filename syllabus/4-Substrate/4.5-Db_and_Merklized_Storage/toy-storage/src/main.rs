

use std::collections::HashMap;

fn main() {
	//  We'll use Strings like Shawn's slides
	let values_to_store = vec!["parity", "participate", "party", "process", "procure", "prospective"];

	// This represents our Key-Value Database. In Substrate this would
	// be RocksDB or ParityDB.
	let mut kvdb = HashMap::new();

	println!("Hello, world!");
}

pub enum NodeType {
	Empty,
	Leaf,
	BranchNoValue,
	BranchWithValue,
}

pub struct Node {
	header: (),
	key: (),
	children: (), // I want to make this simple so we will use binary trees
	// maybe I should have two fields leftChild and rightChild?
	value: (),
}