#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Node
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.
//! - `assertable_node`. Allows token to be used in tests.

#[cfg(feature = "no_std")]
#[cfg_attr(test, macro_use)]
extern crate alloc;

mod native {
	#[cfg(feature = "no_std")]
	pub use alloc::collections::VecDeque;

	#[cfg(feature = "no_std")]
	pub use core::cmp::PartialEq;

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		cmp::PartialEq,
		collections::VecDeque
	};
}

mod abstracts {
	pub use abstract_chearmyp_boundary::{AbstractBoundary, AbstractBoundaryCollection};
	pub use abstract_chearmyp_node::{
		AbstractAttacherCollection,
		AbstractNodeQueue,
		AbstractNode,
		SimpleAbstractNode,
		DynamicAbstractNode
	};
}

mod node_kind {
	pub use abstract_chearmyp_node::NodeKind;
}

/// Contains the types of abstract syntax trees.
mod node;

pub use node::Node;
