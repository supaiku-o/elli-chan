use std::fmt;
use std::vec;

pub mod elli_chan;

fn main() {
	let prime = 7;
	for a in 1..7 {
		for b in 1..7 {
			println!("i: {} j: {} | {}", a, b, elli_chan::ab_constraint(a, b, prime));
		}
		println!("");
	}
}
