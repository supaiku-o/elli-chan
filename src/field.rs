use std::fmt;

use point::Point;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Field {
	pub prime: u64
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({})", self.prime)
	}
}

impl Field {

	pub fn new(prime: u64) -> Field {
		Field {
			prime: prime,
		}
	}

	pub fn new_point(self, x: i64, y: i64, characteristic: i64) -> Point {
		Point::new(x, y, characteristic, self)
	}

	pub fn infinity_point() -> Point {
		Point::infinity()
	}
}
