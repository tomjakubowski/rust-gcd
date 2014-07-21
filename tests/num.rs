extern crate gcd;

#[test]
fn sanity_value_ripped_from_wikipedia() {
    use gcd::gcd;

    assert_eq!(gcd(48, 18), 6i64);
    assert_eq!(gcd(54, 24), 6i64);
    assert_eq!(gcd(48, 180), 12i64);
}
