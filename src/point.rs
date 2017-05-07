use std::fmt;
use std::ops::Add;
use std::cmp::Ordering;

use number::Number;
use field::Field;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
	pub x: Number,
	pub y: Number,
	characteristic: Number
}

impl Point {

	pub fn new(x: i64, y: i64, characteristic: i64, field: Field) -> Point {
		Point {
			x: Number::new(x, field),
			y: Number::new(y, field),
			characteristic: Number::new(characteristic, field)
		}
	}

    pub fn infinity() -> Point {
        Point {
            x: Number::infinity(),
            y: Number::infinity(),
			characteristic: Number::infinity()
        }
    }

	pub fn negative(&self) -> Point {
		Point {
			x: self.x,
			y: self.y.negative(),
			characteristic: Number::infinity()
		}
	}

	pub fn infinite(&self) -> bool {
		self.x.infinite() && self.y.infinite()
	}

	pub fn double(self) -> Point {
		let number3 = Number::new(3, self.x.field);
		let number2 = Number::new(2, self.x.field);
		let mut delta = (number3 * self.x.pow(self.x, number2)) + self.characteristic;
		delta = delta / (number2 * self.y);

		let x3 = delta.pow(delta, number2) - (number2 * self.x);
		let y3 = delta * (self.x - x3) - self.y;

		Point {
			x: x3,
			y: y3,
			characteristic: self.characteristic
		}
	}

	pub fn point_multiplication(point: Point, n: u64) -> Point {
		if n == 0 {
			return Point::infinity();
		} else if n == 1 {
			return point;
		} else if n % 2 == 1 {
			return (point + Point::point_multiplication(point, n - 1)).expect(":(");
		} else {
			return Point::point_multiplication(point.double(), n / 2);
		}
	}
}

impl Add for Point {
	type Output = Option<Point>;
	fn add(self, other: Point) -> Option<Point> {

		if self == other || self == other.negative() {
			None

		} else if other.infinite() {
			Some(self)

		} else {
            let delta = (other.y - self.y) / (other.x - self.x);
			let x3 = (delta * delta) - self.x - other.x;
			let y3 = delta * (self.x - x3) - self.y;

			Some(Point {
				x: x3,
				y: y3,
				characteristic: self.characteristic
			})
		}
	}
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.x.cmp(&other.x)
	}
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.infinite() {
			write!(f, "∞")
		} else {
			write!(f, "({}, {})", self.x, self.y)
		}
	}
}

impl fmt::Debug for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.infinite() {
			write!(f, "∞")
		} else {
			write!(f, "({}, {})", self.x, self.y)
		}
	}
}
