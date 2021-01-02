mod node;
mod scope_stack;

pub use node::Node;

use alloc::vec::Vec;
use crate::lex::{Token, TokenQueue};
use scope_stack::ScopeStack;

pub fn parse<'a, T>(stream: T) -> Vec<Node<'a>>
where T: 'a + Into<TokenQueue<'a>> {
	let stream = stream.into();
	let mut scope_stack = ScopeStack::new();

	for token in stream {
		match token {
		}
	}

	unimplemented!()
}