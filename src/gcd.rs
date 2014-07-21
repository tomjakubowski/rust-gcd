pub fn gcd<T: Int>(a: T, b: T) -> T {
    use std::num::Zero;
    if b == Zero::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}
