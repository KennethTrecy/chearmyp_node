use crate::native::VecDeque;
use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	SimpleAbstractNode
};
use crate::node_kind::NodeKind;
use super::Node;

impl<T, U, V>
SimpleAbstractNode<T, U, V, Node<U, V>, VecDeque<Node<U, V>>, VecDeque<Node<U, V>>>
for Node<U, V>
where
	U: AbstractBoundary<T>,
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

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use super::{SimpleAbstractNode, Node, NodeKind};

	type T = Range<u8>;
	type U = Vec<T>;
	type V = Node<T, U>;
	type W = VecDeque<V>;

	#[test]
	fn can_create_simplex_and_confirm() {
		let simplex = V::new_simplex(0..1, W::new());

		let kind = simplex.kind();

		assert_eq!(kind, NodeKind::Simplex);
	}

	#[test]
	fn can_create_complex_and_confirm() {
		let complex = V::new_complex(2..3, W::new(), W::new());

		let kind = complex.kind();

		assert_eq!(kind, NodeKind::Complex);
	}

	#[test]
	fn can_create_attacher_and_confirm() {
		let attacher = V::new_attacher(4..5, 6..7, U::new());

		let kind = attacher.kind();

		assert_eq!(kind, NodeKind::Attacher);
	}

	#[test]
	fn can_create_line_comment_and_confirm() {
		let line_comment = V::new_line_comment(8..9);

		let kind = line_comment.kind();

		assert_eq!(kind, NodeKind::LineComment);
	}

	#[test]
	fn can_create_block_comment_and_confirm() {
		let block_comment = V::new_block_comment(U::new());

		let kind = block_comment.kind();

		assert_eq!(kind, NodeKind::BlockComment);
	}

	#[test]
	fn can_create_line_othertongue_and_confirm() {
		let line_othertongue = V::new_line_othertongue(10..11);

		let kind = line_othertongue.kind();

		assert_eq!(kind, NodeKind::LineOthertongue);
	}

	#[test]
	fn can_create_block_othertongue_and_confirm() {
		let block_othertongue = V::new_block_othertongue(U::new());

		let kind = block_othertongue.kind();

		assert_eq!(kind, NodeKind::BlockOthertongue);
	}
}
