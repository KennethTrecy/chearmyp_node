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



#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::abstracts::SimpleAbstractNode;
	use super::Node;

	type T = Range<u8>;
	type U = Vec<T>;
	type V = Node<T, U>;
	type W = VecDeque<V>;

	#[test]
	fn can_get_label() {
		use crate::abstracts::AbstractAttacherNode;

		let attacher = V::new_attacher(0..1, 2..3, U::new());

		let label = attacher.label();

		assert_eq!(label, &(0..1));
	}

	#[test]
	fn can_get_content() {
		use crate::abstracts::AbstractAttacherNode;

		let attacher = V::new_attacher(4..5, 6..7, U::new());

		let content = attacher.content();

		assert_eq!(content, &(6..7));
	}

	#[test]
	fn can_get_comments() {
		use crate::abstracts::AbstractAttacherNode;

		let attacher = V::new_attacher(8..9, 10..11, U::new());

		let comments = attacher.comments();

		assert_eq!(comments, &U::new());
	}

	#[test]
	#[should_panic]
	fn cannot_get_label() {
		use crate::abstracts::AbstractAttacherNode;

		let line_othertongue = V::new_line_othertongue(8..9);

		line_othertongue.label();
	}

	#[test]
	#[should_panic]
	fn cannot_get_content() {
		use crate::abstracts::AbstractAttacherNode;

		let line_comment = V::new_line_comment(10..11);

		line_comment.content();
	}

	#[test]
	fn can_get_block_comment() {
		use crate::abstracts::AbstractBlockCommentNode;

		let block_comment = V::new_block_comment(vec![ 12..13 ]);

		let block = block_comment.block();

		assert_eq!(block, &vec![ 12..13 ]);
	}

	#[test]
	#[should_panic]
	fn cannot_get_block_comment() {
		use crate::abstracts::AbstractBlockCommentNode;

		let simplex = V::new_simplex(14..15, W::new());

		simplex.block();
	}

	#[test]
	fn can_get_block_othertongue() {
		use crate::abstracts::AbstractBlockOthertongueNode;

		let block_othertongue = V::new_block_othertongue(vec![ 16..17 ]);

		let block = block_othertongue.block();

		assert_eq!(block, &vec![ 16..17 ]);
	}

	#[test]
	#[should_panic]
	fn cannot_get_block_othertongue() {
		use crate::abstracts::AbstractBlockOthertongueNode;

		let simplex = V::new_simplex(18..19, W::new());

		simplex.block();
	}

	#[test]
	fn can_get_line_comment() {
		use crate::abstracts::AbstractLineCommentNode;

		let line_comment = V::new_line_comment(20..21);

		let line = line_comment.line();

		assert_eq!(line, &(20..21));
	}

	#[test]
	#[should_panic]
	fn cannot_get_line_comment() {
		use crate::abstracts::AbstractLineCommentNode;

		let complex = V::new_complex(22..23, W::new(), W::new());

		complex.line();
	}

	#[test]
	fn can_get_line_othertongue() {
		use crate::abstracts::AbstractLineOthertongueNode;

		let line_othertongue = V::new_line_othertongue(24..25);

		let line = line_othertongue.line();

		assert_eq!(line, &(24..25));
	}

	#[test]
	#[should_panic]
	fn cannot_get_line_othertongue() {
		use crate::abstracts::AbstractLineOthertongueNode;

		let simplex = V::new_simplex(26..27, W::new());

		simplex.line();
	}

	#[test]
	fn can_get_complex_name() {
		use crate::abstracts::AbstractComplexNode;

		let complex = V::new_complex(30..31, W::new(), W::new());

		let name = complex.name();

		assert_eq!(name, &(30..31));
	}

	#[test]
	#[should_panic]
	fn cannot_get_complex_name() {
		use crate::abstracts::AbstractComplexNode;

		let line_comment = V::new_line_comment(32..33);

		line_comment.name();
	}

	#[test]
	fn can_get_simplex_name() {
		use crate::abstracts::AbstractSimplexNode;

		let simplex = V::new_simplex(34..35, W::new());

		let name = simplex.name();

		assert_eq!(name, &(34..35));
	}

	#[test]
	#[should_panic]
	fn cannot_get_simplex_name() {
		use crate::abstracts::AbstractSimplexNode;

		let line_comment = V::new_line_comment(36..37);

		line_comment.name();
	}
}
