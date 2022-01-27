use crate::pos::{IWidth, Pos};

/// Represents a span with an inclusive start ([`Span::low`]) and an exclusive
/// end ([`Span::high`]).
pub struct Span {
	/// Inclusive start of the span.
	low: Pos,

	/// Exclusive end of the span.
	high: Pos,
}

impl Span {
	/// Creates a new span from the given positions.
	///
	/// # Note
	///
	/// If `low` is greater than `high` the values will be switched.
	pub fn new(mut low: Pos, mut high: Pos) -> Self {
		if low > high {
			std::mem::swap(&mut low, &mut high);
		}

		Self { low, high }
	}

	/// Replaces [`Span::low`] with the given value.
	pub fn with_low(self, low: Pos) -> Self {
		let Self { high, .. } = self;
		Self::new(low, high)
	}

	/// Replaces [`Span::high`] with the given value.
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
	pub unsafe fn unchecked_shift_by(self, amount: IWidth) -> Self {
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
	pub unsafe fn unchecked_shift_low_by(self, amount: IWidth) -> Self {
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
	pub unsafe fn unchecked_shift_high_by(self, amount: IWidth) -> Self {
		let Self { low, high } = self;
		let high = high.as_u32();

		let amount_is_neg = amount.is_negative();
		let abs_amount = amount.abs() as u32;

		let high =
			if amount_is_neg { high - abs_amount } else { high - abs_amount };

		Self::new(low, Pos::from_u32(high))
	}
}
