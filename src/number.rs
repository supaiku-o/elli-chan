use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;

use field::Field;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Number  {
    pub value: i64,
    pub field: Field,
}

impl Number {
    pub fn new(value: i64, field: Field) -> Number {
        Number {
            value, 
            field: field,
        }
    }

    pub fn infinity() -> Number {
        Number::new(i64::max_value(), Field::new(0))
    }

    pub fn infinite(&self) -> bool {
        self.value == i64::max_value()
    }

    pub fn negative(&self) -> Number {
		Number {
			value: -self.value,
			field: self.field
		}
    }

}

impl Add for Number {
    type Output = Number;

	fn add(self, other: Number) -> Number {

		let prime = self.field.prime as i64;
		let l = (self.value) % prime;
		let r = (other.value) % prime;

		let mut result = l - prime + r;
		if result < 0 {
			result += prime;
		}

        Number::new(result, self.field)
	}
}

impl Sub for Number {
    type Output = Number;

	fn sub(self, other: Number) -> Number {
		let prime = self.field.prime as i64;
		let l = (self.value) % prime;
		let r = (other.value) % prime;

		let mut result = l - r;
		if result < 0 {
			result += prime;
		}

		Number::new(result, self.field)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
		let prime = self.field.prime as i64;
		let l = (self.value as i64) % prime;
		let r = (other.value as i64) % prime;

		let mut min = cmp::min(l, r);
		let mut max = cmp::max(l, r) as i64;
		let mut result = Number::new(0, self.field);

		while min > 0 {
			if min & (1 << 0) == 1 {
				result = Number::new(max, self.field) + result;
			}

			max = (Number::new(max, self.field) + Number::new(max, self.field)).value;
			min >>= 1;
		}

		result
	}

}

impl Number {

	pub fn pow(&self, base: Number, exp: Number) -> Number {
        let prime = self.field.prime;

        let mut numeric_result = 1;
        let mut numeric_base = base.value;
        let mut numeric_exp = exp.value;
		numeric_base %= prime as i64;

		while numeric_exp > 0 {
			if numeric_exp % 2 == 1 {
				numeric_result = (numeric_result * numeric_base) % prime as i64;
			}

			numeric_exp >>= 1;
			numeric_base = (numeric_base * numeric_base) % prime as i64;
		}

		Number::new(numeric_result, self.field)
	}

}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        let prime = (self.field.prime - 2) as i64;
		self * other.pow(other, Number::new(prime, self.field))
	}
}

impl Ord for Number {
    fn cmp(&self, other: &Number) -> Ordering {
        self.value.cmp(&other.value)
	}
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.infinite() {
			write!(f, "∞")
		} else {
			write!(f, "{}", self.value)
		}
	}
}

impl fmt::Debug for Number {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.infinite() {
			write!(f, "∞")
		} else {
			write!(f, "{}", self.value)
		}
	}
}