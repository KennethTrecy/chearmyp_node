use crate::native::{PartialEq, VecDeque};
use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	SimpleAbstractNode
};
use crate::node_kind::NodeKind;
use super::Node;

impl<T, U, V>
SimpleAbstractNode<T, U, V, U, Node<U, V>, VecDeque<Node<U, V>>, VecDeque<Node<U, V>>>
for Node<U, V>
where
	U: AbstractBoundary<T> + PartialEq,
	V: AbstractBoundaryCollection<T, U> {
	fn kind(&self) -> NodeKind {
		match self {
			Self::Simplex(_, _) => NodeKind::Simplex,
			Self::Complex(_, _, _) => NodeKind::Complex,
			Self::Attacher(_, _, _) => NodeKind::Attacher,
			Self::LineComment(_) => NodeKind::LineComment,
			Self::BlockComment(_) => NodeKind::BlockComment,
			Self::LineOthertongue(_) => NodeKind::LineOthertongue,
			Self::BlockOthertongue(_) => NodeKind::BlockOthertongue
		}
	}

	fn new_line_comment(line: U) -> Self {
		Node::LineComment(line)
	}

	fn new_line_othertongue(line: U) -> Self {
		Node::LineOthertongue(line)
	}

	fn new_block_comment(collection: V) -> Self {
		Node::BlockComment(collection)
	}

	fn new_block_othertongue(collection: V) -> Self {
		Node::BlockOthertongue(collection)
	}

	fn new_attacher(label: U, content: U, comments: V) -> Self {
		Node::Attacher(label, content, comments)
	}

	fn new_simplex(name: U, attachers: VecDeque<Node<U, V>>) -> Self {
		Node::Simplex(name, attachers)
	}

	fn new_complex(name: U, attachers: VecDeque<Node<U, V>>, nodes: VecDeque<Node<U, V>>) -> Self {
		Node::Complex(name, attachers, nodes)
	}
}
