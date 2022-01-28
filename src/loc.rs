use std::fmt;

use crate::pos::Pos;

/// A location inside a [`Source`](`crate::src::Source`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc {
	/// Zero indexed line.
	pub line: usize,

	/// Zero indexed column.
	pub column: Pos,
}

impl fmt::Display for Loc {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.line + 1, self.column.as_u32() + 1)
	}
}
