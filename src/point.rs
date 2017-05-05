use std::fmt;
use std::ops::Add;
use std::cmp::Ordering;

use number::Number;
use field::Field;

#[derive(Eq, PartialEq, Hash)]
pub struct Point {
	pub x: Number,
	pub y: Number
}

impl Point {

	pub fn new(x: i64, y: i64, field: &Field) -> Point {
		Point {
			x: Number::new(x, &field),
			y: Number::new(y, &field),
		}
	}

    pub fn infinity() -> Point {
        Point {
            x: Number::infinity(),
            y: Number::infinity()
        }
    }

	pub fn negative(&self) -> Point {
		Point {
			x: self.x.dup(),
			y: self.y.negative()
		}
	}

	pub fn infinite(&self) -> bool {
		self.x.infinite() && self.y.infinite()
	}
}

impl Add for Point {
	type Output = Option<Point>;
	fn add(self, other: Point) -> Option<Point> {

		if self == other || self == other.negative() {
			None
		} else {
            println!("{:?} + {:?}", self, other);
            let delta = (other.y - self.y.dup()) / (other.x.dup() - self.x.dup());
			let x3 = (delta.dup() * delta.dup()) - self.x.dup() - other.x.dup();
			let y3 = delta * (self.x.dup() - x3.dup()) - self.y.dup();

			Some(Point {
				x: x3,
				y: y3
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
