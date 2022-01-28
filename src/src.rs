use std::path::PathBuf;

/// Determins the origin from which a [`Source`] came from.
///
/// This is mainly used when printing to the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Origin {
	/// The origin is a path.
	Path(PathBuf),

	/// The origin is a named document.
	Named(String),

	/// The origin is not known.
	Unknown,
}

/// A special width character is any character which when printed is not
/// displayed as as the same length.
///
/// E.g. `\t` is a single character but will/can be displayed as 4 spaces.
///
/// They are recorded to correct any position when printing to the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialWidthChar {}

/// A source for which to show/attach diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Source {
	/// Origin of the source.
	origin: Origin,

	/// Acutual data the source contains.
	data: String,

	/// Indices for each line start.
	line_indices: Vec<usize>,

	/// List of special width characters.
	swc: Vec<SpecialWidthChar>,
}
