use crate::native::VecDeque;

/// Contains the nodes used for parsing.
#[cfg_attr(feature = "assertable_node", derive(Debug, PartialEq))]
pub enum Node<T, U> {
	LineComment(T),
	BlockComment(U),
	Simplex(T, VecDeque<Node<T, U>>),
	Complex(T, VecDeque<Node<T, U>>, VecDeque<Node<T, U>>),
	Attacher(T, T, U),
	LineOthertongue(T),
	BlockOthertongue(U)
}

mod simple_abstract_node;
mod dynamic_abstract_node;
