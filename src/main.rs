pub mod elliptic_curve;
pub mod field;
pub mod point;
pub mod number;
pub mod helper;

fn main() {
	let prime = 29;
	let a = 4;
	let b = 20;

	elliptic_curve::EllipticCurve::new(prime)
		.with(a, b)
		.calculate()
		.print();
}
