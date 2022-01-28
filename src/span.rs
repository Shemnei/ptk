use std::ops::{Index, Range};

use crate::pos::{IWidth, Pos};

/// Represents a span with an inclusive start ([`Span::low`]) and an exclusive
/// end ([`Span::high`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
	/// Inclusive start of the span.
	pub low: Pos,

	/// Exclusive end of the span.
	pub high: Pos,
}

impl Span {
	/// Creates a new span from the given positions.
	///
	/// # Note
	///
	/// If `low` is greater than `high` the values will be switched.
	#[must_use]
	pub fn new(mut low: Pos, mut high: Pos) -> Self {
		if low > high {
			std::mem::swap(&mut low, &mut high);
		}

		Self { low, high }
	}

	/// Replaces [`Span::low`] with the given value.
	#[must_use]
	pub fn with_low(self, low: Pos) -> Self {
		let Self { high, .. } = self;
		Self::new(low, high)
	}

	/// Replaces [`Span::high`] with the given value.
	#[must_use]
	pub fn with_high(self, high: Pos) -> Self {
		let Self { low, .. } = self;
		Self::new(low, high)
	}

	/// Shifts both [`Span::low`] and [`Span::high`] by the given amount.
	///
	/// # Panics
	///
	/// This function will panic if an under or overflow occures during the
	/// shifting.
	#[must_use]
	pub fn shift_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let (low, high) = (low.as_u32(), high.as_u32());

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let (low, high) = if amount_is_neg {
			(
				low.checked_sub(abs_amount)
					.expect("Width underflow while shifting `low`"),
				high.checked_sub(abs_amount)
					.expect("Width underflow while shifting `high`"),
			)
		} else {
			(
				low.checked_add(abs_amount)
					.expect("Width overflow while shifting `low`"),
				high.checked_add(abs_amount)
					.expect("Width overflow while shifting `high`"),
			)
		};

		Self::new(Pos::from_u32(low), Pos::from_u32(high))
	}

	/// Shifts both [`Span::low`] and [`Span::high`] by the given amount.
	#[must_use]
	pub fn unchecked_shift_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let (low, high) = (low.as_u32(), high.as_u32());

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let (low, high) = if amount_is_neg {
			(low - abs_amount, high - abs_amount)
		} else {
			(low + abs_amount, high + abs_amount)
		};

		Self::new(Pos::from_u32(low), Pos::from_u32(high))
	}

	/// Shifts [`Span::low`] by the given amount.
	///
	/// # Panics
	///
	/// This function will panic if an under or overflow occures during the
	/// shifting.
	#[must_use]
	pub fn shift_low_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let low = low.as_u32();

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let low = if amount_is_neg {
			low.checked_sub(abs_amount)
				.expect("Width underflow while shifting `low`")
		} else {
			low.checked_add(abs_amount)
				.expect("Width overflow while shifting `low`")
		};

		Self::new(Pos::from_u32(low), high)
	}

	/// Shifts both [`Span::low`] by the given amount.
	#[must_use]
	pub fn unchecked_shift_low_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let low = low.as_u32();

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let low =
			if amount_is_neg { low - abs_amount } else { low + abs_amount };

		Self::new(Pos::from_u32(low), high)
	}

	/// Shifts [`Span::high`] by the given amount.
	///
	/// # Panics
	///
	/// This function will panic if an under or overflow occures during the
	/// shifting.
	#[must_use]
	pub fn shift_high_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let high = high.as_u32();

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let high = if amount_is_neg {
			high.checked_sub(abs_amount)
				.expect("Width underflow while shifting `low`")
		} else {
			high.checked_add(abs_amount)
				.expect("Width overflow while shifting `low`")
		};

		Self::new(low, Pos::from_u32(high))
	}

	/// Shifts both [`Span::high`] by the given amount.
	#[must_use]
	pub fn unchecked_shift_high_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let high = high.as_u32();

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let high =
			if amount_is_neg { high - abs_amount } else { high + abs_amount };

		Self::new(low, Pos::from_u32(high))
	}

	/// Combines two spans and creates a new span which encloses both.
	#[must_use]
	pub fn union(self, other: Self) -> Self {
		let low = std::cmp::min(self.low.as_u32(), other.low.as_u32());
		let high = std::cmp::max(self.high.as_u32(), other.high.as_u32());

		Self::new(Pos::from_u32(low), Pos::from_u32(high))
	}

	/// Converts this span to a range.
	pub const fn to_pos_range(self) -> Range<Pos> {
		self.low..self.high
	}

	/// Converts this span to a range.
	pub const fn to_u32_range(self) -> Range<u32> {
		self.low.as_u32()..self.high.as_u32()
	}

	/// Converts this span to a range.
	pub const fn to_usize_range(self) -> Range<usize> {
		self.low.as_usize()..self.high.as_usize()
	}
}

impl<P> From<Range<P>> for Span
where
	P: Into<Pos>,
{
	fn from(value: Range<P>) -> Self {
		Self::new(value.start.into(), value.end.into())
	}
}

impl Index<Span> for str {
	type Output = Self;

	fn index(&self, index: Span) -> &Self::Output {
		&self[index.to_usize_range()]
	}
}

impl Index<Span> for [u8] {
	type Output = Self;

	fn index(&self, index: Span) -> &Self::Output {
		&self[index.to_usize_range()]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let span = Span::new(Pos::from_u32(10), Pos::from_u32(100));
		assert_eq!(span.low.as_u32(), 10);
		assert_eq!(span.high.as_u32(), 100);
	}

	#[test]
	fn new_swapped() {
		let span = Span::new(Pos::from_u32(0xdead), Pos::from_u32(0xbeef));
		assert_eq!(span.low.as_u32(), 0xbeef);
		assert_eq!(span.high.as_u32(), 0xdead);
	}

	#[test]
	fn with_low() {
		let span = Span::new(Pos::from_u32(20), Pos::from_u32(50));
		let span = span.with_low(Pos::from_u32(11));
		assert_eq!(span.low.as_u32(), 11);
		assert_eq!(span.high.as_u32(), 50);

		let span = Span::new(Pos::from_u32(20), Pos::from_u32(50));
		let span = span.with_low(Pos::from_u32(60));
		assert_eq!(span.low.as_u32(), 50);
		assert_eq!(span.high.as_u32(), 60);
	}

	#[test]
	fn with_high() {
		let span = Span::new(Pos::from_u32(20), Pos::from_u32(50));
		let span = span.with_high(Pos::from_u32(44));
		assert_eq!(span.low.as_u32(), 20);
		assert_eq!(span.high.as_u32(), 44);

		let span = Span::new(Pos::from_u32(20), Pos::from_u32(50));
		let span = span.with_high(Pos::from_u32(8));
		assert_eq!(span.low.as_u32(), 8);
		assert_eq!(span.high.as_u32(), 20);
	}

	#[test]
	fn shift_by_pos() {
		let span = Span::new(Pos::from_u32(0), Pos::from_u32(100));
		let span = span.shift_by(20);
		assert_eq!(span.low.as_u32(), 20);
		assert_eq!(span.high.as_u32(), 120);
	}

	#[test]
	fn shift_by_neg() {
		let span = Span::new(Pos::from_u32(21), Pos::from_u32(100));
		let span = span.shift_by(-20);
		assert_eq!(span.low.as_u32(), 1);
		assert_eq!(span.high.as_u32(), 80);
	}

	#[test]
	fn shift_low_by_pos() {
		let span = Span::new(Pos::from_u32(0), Pos::from_u32(100));
		let span = span.shift_low_by(20);
		assert_eq!(span.low.as_u32(), 20);
		assert_eq!(span.high.as_u32(), 100);
	}

	#[test]
	fn shift_low_by_neg() {
		let span = Span::new(Pos::from_u32(21), Pos::from_u32(100));
		let span = span.shift_low_by(-20);
		assert_eq!(span.low.as_u32(), 1);
		assert_eq!(span.high.as_u32(), 100);
	}

	#[test]
	fn shift_high_by_pos() {
		let span = Span::new(Pos::from_u32(0), Pos::from_u32(100));
		let span = span.shift_high_by(20);
		assert_eq!(span.low.as_u32(), 0);
		assert_eq!(span.high.as_u32(), 120);
	}

	#[test]
	fn shift_high_by_neg() {
		let span = Span::new(Pos::from_u32(21), Pos::from_u32(100));
		let span = span.shift_high_by(-20);
		assert_eq!(span.low.as_u32(), 21);
		assert_eq!(span.high.as_u32(), 80);
	}

	#[test]
	fn union() {
		let span_lhs = Span::new(Pos::from_u32(50), Pos::from_u32(80));
		let span_rhs = Span::new(Pos::from_u32(10), Pos::from_u32(80));
		let union = span_lhs.union(span_rhs);
		assert_eq!(union.low.as_u32(), 10);
		assert_eq!(union.high.as_u32(), 80);

		let span_lhs = Span::new(Pos::from_u32(0), Pos::from_u32(80));
		let span_rhs = Span::new(Pos::from_u32(10), Pos::from_u32(100));
		let union = span_lhs.union(span_rhs);
		assert_eq!(union.low.as_u32(), 0);
		assert_eq!(union.high.as_u32(), 100);

		let span_lhs = Span::new(Pos::from_u32(0), Pos::from_u32(100));
		let span_rhs = Span::new(Pos::from_u32(10), Pos::from_u32(80));
		let union = span_lhs.union(span_rhs);
		assert_eq!(union.low.as_u32(), 0);
		assert_eq!(union.high.as_u32(), 100);

		let span_lhs = Span::new(Pos::from_u32(20), Pos::from_u32(100));
		let span_rhs = Span::new(Pos::from_u32(10), Pos::from_u32(120));
		let union = span_lhs.union(span_rhs);
		assert_eq!(union.low.as_u32(), 10);
		assert_eq!(union.high.as_u32(), 120);
	}
}
