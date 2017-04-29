// y² = x³ + ax + b
pub fn elliptic_curve(x: u32, a: u32, b: u32) -> u32 {
	x.pow(3) + a * x + b
}

// 4a³ + 27b² % p == 0
pub fn ab_constraint(a: u32, b: u32, p: u32) -> bool {
	(4 * a.pow(3) + 27 * b.pow(2)) % p == 0
}

pub fn field_n(prime: u32) -> Vec<u32> {

	vec![1, 2]
}

// http://stackoverflow.com/a/1801446
pub fn is_prime(number: u32) -> bool {
	let prime = match number {
		2 | 3 => true,
		number if number % 2 == 0 => false,
		number if number % 3 == 0 => false,
		_ => false
	};

	if !prime {
		let mut i = 5;
		let mut w = 2;
		while i * i <= number {
			if number % i == 0 {
				return false;
			}

			i += w;
			w = 6 - w;
		}
	}

	true
}
