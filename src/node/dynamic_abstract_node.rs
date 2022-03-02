use crate::native::VecDeque;
use crate::abstracts::DynamicAbstractNode;
use super::Node;

impl<T, U> DynamicAbstractNode for Node<T, U> {
	type Name = T;
	type Attachers = VecDeque<Node<T, U>>;
	type Nodes = VecDeque<Node<T, U>>;
	type Line = T;
	type Block = U;
	type Label = T;
	type Content = T;
	type Comments = U;

	fn name(&self) -> &Self::Name {
		match self {
			Self::Simplex(name, _) | Self::Complex(name, _, _) => name,
			_ => panic!("Cannot retrieve the name because it is not a simplex or complex node.")
		}
	}

	fn attachers(&self) -> &Self::Attachers {
		match self {
			Self::Simplex(_, attachers) | Self::Complex(_, attachers, _) => attachers,
			_ => panic!("Cannot retrieve the attachers because it is not a simplex or complex node.")
		}
	}

	fn nodes(&self) -> &Self::Nodes {
		match self {
			Self::Complex(_, _, nodes) => nodes,
			_ => panic!("Cannot retrieve the nodes because it is not a complex node.")
		}
	}

	fn line(&self) -> &Self::Line {
		match self {
			Self::LineComment(line) | Self::LineOthertongue(line) => line,
			_ => panic!("Cannot retrieve the line because it is not a line comment/othertongue node.")
		}
	}

	fn block(&self) -> &Self::Block {
		match self {
			Self::BlockComment(block) | Self::BlockOthertongue(block) => block,
			_ => panic!(
				"Cannot retrieve the block because it is not a block comment/othertongue node.")
		}
	}

	fn label(&self) -> &Self::Label {
		match self {
			Self::Attacher(label, _, _) => label,
			_ => panic!("Cannot retrieve the label because it is not a attacher node.")
		}
	}

	fn content(&self) -> &Self::Content {
		match self {
			Self::Attacher(_, content, _) => content,
			_ => panic!("Cannot retrieve the content because it is not a attacher node.")
		}
	}

	fn comments(&self) -> &Self::Comments {
		match self {
			Self::Attacher(_, _, comments) => comments,
			_ => panic!("Cannot retrieve the comments because it is not a attacher node.")
		}
	}
}
