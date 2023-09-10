use crate::{challenge_4_4, challenge_4_4_1, challenge_4_4_2};
use fn_traits::Fn;

pub fn safe_root_reciprocal(x: f64) -> Option<f64> {
    challenge_4_4_1::compose(challenge_4_4::safe_root, challenge_4_4_2::safe_reciprocal).call((x,))
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
