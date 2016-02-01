// fn main() {
//     println!("Hello, world!");
// }


use std::cmp::Ordering::*;

pub struct BST<K,V>(NodePtr<K,V>);

struct Node<K,V>{
	key:K,
	value:V,
	left:NodePtr<K,V>,
	right:NodePtr<K,V>,
}

type NodePtr<K,V> = Option<Box<Node<K,V>>>;

impl<K,V> BST<K,V>{
	pub fn new()->Self{
		BST(None)
	}
}