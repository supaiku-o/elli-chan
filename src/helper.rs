// http://stackoverflow.com/a/1801446
pub fn is_prime(number: u64) -> bool {
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
