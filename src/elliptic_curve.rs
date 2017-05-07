use std::collections::HashMap;
use std::vec::Vec;

use helper;
use point::Point;
use field::Field;

pub struct EllipticCurve {
	pub field: Field,
	pub points: Vec<Point>,
	pub a: i64,
	pub b: i64
}

impl EllipticCurve {

	pub fn new(prime: u64) -> EllipticCurve {
		EllipticCurve {
			field: Field::new(prime),
			a: 0,
			b: 0,
			points: Vec::new(),
		}
	}

	pub fn with(mut self, a: i64, b: i64) -> EllipticCurve {
		assert!(self.discriminant_not_zero(a, b));

		self.a = a;
		self.b = b;
	
		self
	}

	pub fn calculate(mut self) -> EllipticCurve {
		let mut lookup_table = HashMap::new();

		for x in 0..self.field.prime {
			let x_squared = (x * x) % self.field.prime;
			let entry = lookup_table.entry(x_squared).
									or_insert(vec![]);
			entry.push(x);
		}

		for x in 0..self.field.prime {
			let result = self.calculate_y(x as i64);
			if let Some(ys) = lookup_table.get(&result) {
				for &y in ys {
					self.points.push(self.field.new_point(x as i64,y as i64, self.a));
				}
			}
			else {
				if !self.has_point(&Field::infinity_point()) {
					self.points.push(Field::infinity_point());
				}
			}
		}

		self.points.sort();

		self
	}

	pub fn order(&self) -> usize {
		self.points.len()
	}

	pub fn is_cyclic(&self) -> bool {
		helper::is_prime(self.order() as u64)
	}

	pub fn print(&self) {
		println!("Found the following {} points on the curve x³ + {}x + {} over the Field F{}:", self.order(), self.a, self.b, self.field.prime);
		println!("\n");

		let mut i = 0;
		for point in &self.points {
			i += 1;
			print!("P{}: {}\t", i, point);

			if i % 5 == 0 {
				print!("\n");
			}
		}

		println!("\n");
	}

	pub fn has_point(&self, point: &Point) -> bool {
		self.points.iter().any(|p| p == point)
	}

	// y² = x³ + ax + b
	fn calculate_y(&self, x: i64) -> u64 {
		((x.pow(3) + self.a * x + self.b) % self.field.prime as i64) as u64
	}

	fn discriminant_not_zero(&self, a: i64, b: i64) -> bool {
		let r = 4 * a.pow(3) + 27 * b.pow(2);
		-16 * r % (self.field.prime as i64) != 0
	}
}
