extern crate elli_chan;

use elli_chan::helper;
use elli_chan::number::Number;
use elli_chan::point::Point;
use elli_chan::field::Field;
use elli_chan::elliptic_curve::EllipticCurve;

#[test]
fn is_prime() {
	assert!(helper::is_prime(2));
	assert!(helper::is_prime(7));
	assert!(helper::is_prime(3));
	assert!(helper::is_prime(13));
	assert!(helper::is_prime(29));
}

#[test]
fn check_math() {
	let f29 = Field { prime: 29 };

	let n1 = Number::new(17, f29);
	let n2 = Number::new(20, f29);
	assert_eq!((n1 + n2).value, 8);
	assert_eq!((n1 - n2).value, 26);
	assert_eq!((n1 * n2).value, 21);
	assert_eq!((Number::new(1, f29) / n1).value, 12);
}

#[test]
fn points_on_given_curve_are_37() {
	assert_eq!(EllipticCurve::new(29)
				.with(4, 20)
				.calculate()
				.order(), 37);
}

#[test]
fn elliptic_curve_addition() {
	let ec = EllipticCurve::new(29)
		.with(4, 20).calculate();

	let p1 = Point::new(5, 22, ec.a, ec.field);
	let p2 = Point::new(16, 27, ec.a, ec.field);

	assert!(ec.has_point(&p1));
	assert!(ec.has_point(&p2));

	let p3 = (p1 + p2).unwrap();
	assert_eq!(p3, Point::new(13, 6, ec.a, ec.field));

	assert!(ec.has_point(&p3));
}

#[test]
fn elliptic_curve_multiplication() {
	let ec = EllipticCurve::new(29)
		.with(4, 20).calculate();

	let p1 = Point::new(1, 5, ec.a, ec.field);

	for i in 0..ec.order() {
		let pi = Point::point_multiplication(p1, i as u64);
		if i == 0 {
			assert_eq!(pi, Point::infinity());
		}

		assert!(ec.has_point(&pi));
	}
}