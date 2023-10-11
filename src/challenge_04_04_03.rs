use crate::{challenge_04_04, challenge_04_04_01, challenge_04_04_02};
use fn_traits::Fn;

pub fn safe_root_reciprocal(x: f64) -> Option<f64> {
    challenge_04_04_01::compose(
        challenge_04_04::safe_root,
        challenge_04_04_02::safe_reciprocal,
    )
    .call((x,))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_safe_reciprocal() {
        assert_eq!(super::safe_root_reciprocal(-1.0), None);
        assert_eq!(super::safe_root_reciprocal(0.0), None);
        assert_eq!(super::safe_root_reciprocal(0.25), Some(2.0));
        assert_eq!(super::safe_root_reciprocal(1.0), Some(1.0));
        assert_eq!(super::safe_root_reciprocal(4.0), Some(0.5));
    }
}
