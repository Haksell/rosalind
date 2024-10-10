#[cfg(test)]
pub fn is_close<T: Into<f64>>(a: T, b: T, epsilon: T) -> bool {
    (a.into() - b.into()).abs() <= epsilon.into()
}

#[cfg(test)]
macro_rules! assert_is_close {
    ($a:expr, $b:expr, $epsilon:expr) => {
        if !$crate::floats::is_close($a, $b, $epsilon) {
            panic!(
                "assertion failed: `(left ≈ right)` (left: `{:?}`, right: `{:?}`, epsilon: `{:?}`)",
                $a, $b, $epsilon
            );
        }
    };
    ($a:expr, $b:expr) => {
        assert_is_close!($a, $b, 1e-6);
    };
}
