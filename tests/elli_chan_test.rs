extern crate elli_chan;

use elli_chan::elli_chan as elliptic;

#[test]
fn is_prime() {
	assert!(elliptic::is_prime(2));
	assert!(elliptic::is_prime(7));
	assert!(elliptic::is_prime(3));
	assert!(elliptic::is_prime(13));
	assert!(elliptic::is_prime(29));
}
