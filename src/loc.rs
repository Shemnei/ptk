use crate::pos::Pos;

/// A location inside a [`Source`](`crate::src::Source`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc {
	/// Zero indexed line.
	line: usize,

	/// Zero indexed column.
	column: Pos,
}
