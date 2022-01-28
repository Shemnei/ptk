pub type Width = u32;
pub type IWidth = i32;

/// Represents a position.
///
/// This is mainly used to keep track of character or byte positions in a
/// source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos(pub Width);

impl Pos {
	/// Creates a new position from a [`u32`].
	pub const fn from_u32(value: u32) -> Self {
		Self(value)
	}

	/// Creates a new position from a [`usize`].
	pub const fn from_usize(value: usize) -> Self {
		Self(value as u32)
	}

	/// Returns this position as a [`u32`].
	pub const fn as_u32(self) -> u32 {
		self.0
	}

	/// Returns this position as a [`usize`].
	pub const fn as_usize(self) -> usize {
		self.0 as usize
	}
}

impl From<u32> for Pos {
	fn from(value: u32) -> Self {
		Self::from_u32(value)
	}
}

impl From<usize> for Pos {
	fn from(value: usize) -> Self {
		Self::from_usize(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_u32() {
		let value = u32::MIN;
		let pos = Pos::from_u32(value);
		assert_eq!(pos.0, value);

		let value = u32::MAX;
		let pos = Pos::from_u32(value);
		assert_eq!(pos.0, value);

		let value = 0xdeadbeef;
		let pos = Pos::from_u32(value);
		assert_eq!(pos.0, value);
	}

	#[test]
	fn from_usize() {
		let value = u32::MIN;
		let pos = Pos::from_usize(value as usize);
		assert_eq!(pos.0, value);

		let value = u32::MAX;
		let pos = Pos::from_usize(value as usize);
		assert_eq!(pos.0, value);

		let value = 0xdeadbeef;
		let pos = Pos::from_usize(value as usize);
		assert_eq!(pos.0, value);
	}

	#[test]
	fn as_u32() {
		let value = u32::MIN;
		let pos = Pos::from_u32(value);
		assert_eq!(pos.as_u32(), value);

		let value = u32::MAX;
		let pos = Pos::from_u32(value);
		assert_eq!(pos.as_u32(), value);

		let value = 0xdeadbeef;
		let pos = Pos::from_u32(value);
		assert_eq!(pos.as_u32(), value);
	}

	#[test]
	fn as_usize() {
		let value = u32::MIN as usize;
		let pos = Pos::from_usize(value);
		assert_eq!(pos.as_usize(), value);

		let value = u32::MAX as usize;
		let pos = Pos::from_usize(value);
		assert_eq!(pos.as_usize(), value);

		let value = 0xdeadbeef_usize;
		let pos = Pos::from_usize(value);
		assert_eq!(pos.as_usize(), value);
	}
}
