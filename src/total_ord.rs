/// Implement Ord on struct that only implements a PartialOrd.
///
/// # Examples
///
/// ```
/// use procon::TotalOrd;
///
/// fn main() {
///     let mut v: Vec<f32> = vec![0.9, 42.0, 42.0, -3.2, 100.0, 42.0];
///
///     v.sort_by_key(|&x| TotalOrd(x));
///
///     assert_eq!(v, vec![-3.2, 0.9, 42.0, 42.0, 42.0, 100.0,]);
/// }
/// ```
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TotalOrd<T>(pub T);

impl<T: PartialEq> Eq for TotalOrd<T> {}

impl<T: PartialOrd> Ord for TotalOrd<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ord) = self.0.partial_cmp(&other.0) {
            ord
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
