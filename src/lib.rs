mod total_ord;

pub use total_ord::*;

#[doc(hidden)]
pub use std::io::*;

/// Receive inputs up to EOF.
///
/// # Examples
///
/// ```
/// use procon::*;
///
/// fn main() {
///     let source = "4 -3 4.5\n0 1 2 3 4 5";
///     let mut reader = std::io::BufReader::new(source.as_bytes());
///
///     read_to_end!(reader, a: usize, b: i32, c: f64, x: [i32; a]);
///
///     assert_eq!(a, 4);
///     assert_eq!(b, -3);
///     assert_eq!(c, 4.5);
///     assert_eq!(x, vec![0, 1, 2, 3]);
/// }
/// ```
#[macro_export]
macro_rules! read_to_end {
    ($s:expr, $($r:tt)*) => {
        let mut buf: Vec<u8> = Vec::new();
        ($s).read_to_end(&mut buf).unwrap_or_else(|_| panic!("failed to read_to_end"));
        let mut iter = unsafe { std::str::from_utf8_unchecked(&buf) }.split_whitespace();
        read_iterator!(iter, $($r)*);
    };
}

/// Receives a specified number of lines of input.
///
/// # Examples
///
/// ```
/// use procon::*;
///
/// fn main() {
///     let source = "4 -3 4.5\n0 1 2 3 4 5";
///     let mut reader = std::io::BufReader::new(source.as_bytes());
///
///     read_lines!(reader, 2, a: usize, b: i32, c: f64, x: [i32; a]);
///
///     assert_eq!(a, 4);
///     assert_eq!(b, -3);
///     assert_eq!(c, 4.5);
///     assert_eq!(x, vec![0, 1, 2, 3]);
/// }
/// ```
#[macro_export]
macro_rules! read_lines {
    ($s:expr, $n:expr, $($r:tt)*) => {
        let mut buf: Vec<u8> = Vec::new();
        for _ in 0..($n) {
            ($s).read_until(b'\n', &mut buf).unwrap_or_else(|_| panic!("failed to read_until"));
        };
        let mut iter = unsafe { std::str::from_utf8_unchecked(&buf) }.split_whitespace();
        read_iterator!(iter, $($r)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_iterator {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $v:ident : $t:tt $($r:tt)*) => {
        #[allow(unused_mut)]
        #[allow(clippy::needless_collect)]
        let mut $v = read_value!($iter, $t);
        read_iterator!($iter $($r)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    ($iter:expr, ($($t:tt),*)) => {
        ($(read_value!($iter, $t)),*)
    };

    ($iter:expr, [$t:tt ; $len:expr]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, $t:ty) => {
        $iter.next()
            .unwrap_or_else(|| panic!("failed to next"))
            .parse::<$t>()
            .unwrap_or_else(|_| panic!("failed to parse"))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_read_vec2() {
        let source = "1 2 3 4 5 6";
        let mut reader = std::io::BufReader::new(source.as_bytes());

        read_to_end!(reader, a: [[usize; 3]; 2]);

        assert_eq!(a.len(), 2);
        assert_eq!(a, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_read_chars() {
        let source = "Hello!";
        let mut reader = std::io::BufReader::new(source.as_bytes());

        read_to_end!(reader, a: chars);

        assert_eq!(a, vec!['H', 'e', 'l', 'l', 'o', '!']);
    }
}
