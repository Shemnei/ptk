use std::fmt;

/// A location inside a [`Source`](`crate::src::Source`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc {
	/// Zero indexed line.
	pub line: usize,

	/// Zero indexed column.
	pub column: usize,
}

impl Loc {
	pub fn new(line: usize, column: usize) -> Self {
		Self { line, column }
	}
}

impl fmt::Display for Loc {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.line + 1, self.column + 1)
	}
}
