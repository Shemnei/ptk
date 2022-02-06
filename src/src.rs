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

/// A source for which to show/attach diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Source {
	/// Origin of the source.
	origin: Origin,

	/// Acutual data the source contains.
	data: String,

	/// Indices for each line start.
	line_indices: Vec<usize>,
	// TODO: special width chars to get correct index when printing
}

impl Source {
	pub fn new(origin: Origin, data: String) -> Self {
		let line_indices = scan_lines(&data);

		Self { origin, data, line_indices }
	}

	pub fn from_file(path: PathBuf) -> Result<Self, std::io::Error> {
		let data = std::fs::read_to_string(&path)?;
		Ok(Self::new(Origin::Path(path), data))
	}
}

fn scan_lines(mut data: &str) -> Vec<usize> {
	let mut line_indices = vec![0];
	let mut offset = 0;

	while let Some(index) = data.find('\n') {
		let absolute_index = offset + index;

		line_indices.push(absolute_index);

		let skip_to_index = index + 1;
		data = &data[skip_to_index..];
		offset += skip_to_index;
	}

	line_indices
}

#[cfg(test)]
mod tests {
	#[test]
	fn scan_lines() {
		const DATA: &str = "Hello\nWorld\n";

		assert_eq!(super::scan_lines(DATA), vec![0, 5, 11]);
		assert_eq!(DATA.as_bytes()[5], b'\n');
		assert_eq!(DATA.as_bytes()[11], b'\n');
	}
}
