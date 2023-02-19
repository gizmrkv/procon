use super::*;

/// Get GCD of a and b.
///
/// **Complexity** 'O(log(max(a, b)))'
///
/// # Example
///
/// ```
/// use procon::math::gcd;
/// assert_eq!(gcd(630, 300), 30);
/// assert_eq!(gcd(300, 630), 30);
/// ```
pub fn gcd<T: Num>(mut a: T, mut b: T) -> T {
    loop {
        let r = a % b;
        if r == T::zero() {
            break b;
        }
        (a, b) = (b, r)
    }
}

/// Get GCD of a and b with extended Euclidean Algorithm.
///
/// **Complexity** 'O(log(max(a, b)))'
///
/// # Example
///
/// ```
/// use procon::math::extgcd;
/// assert_eq!(extgcd(13, 5), (2, -5));
/// assert_eq!(extgcd(5, 13), (-5, 2));
/// ```
pub fn extgcd<T: Num>(mut a: T, mut b: T) -> (T, T) {
    let (mut xa, mut ya) = (T::one(), T::zero());
    let (mut xb, mut yb) = (T::zero(), T::one());
    while a % b != T::zero() {
        let temp = a / b;
        let c = a - b * temp;
        let xc = xa - xb * temp;
        let yc = ya - yb * temp;
        (a, xa, ya) = (b, xb, yb);
        (b, xb, yb) = (c, xc, yc);
    }
    (xb, yb)
}
