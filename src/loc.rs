use crate::pos::Pos;

/// A location inside a [`Source`](`crate::src::Source`).
pub struct Loc {
    /// Zero indexed line.
	line: usize,

    /// Zero indexed column.
	column: Pos,
}
