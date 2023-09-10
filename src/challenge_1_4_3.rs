#[cfg(test)]
mod tests {
    use crate::{challenge_1_4_1, challenge_1_4_2};
    use fn_traits::{Fn, FnMut, FnOnce};

    #[test]
    fn test_compose_identity() {
        let plus_2 = |x: u32| x + 2;

        assert_eq!(
            challenge_1_4_2::compose(challenge_1_4_1::id, plus_2).call_once((2,)),
            4,
        );

        assert_eq!(
            challenge_1_4_2::compose(challenge_1_4_1::id, plus_2).call_mut((2,)),
            4,
        );

        assert_eq!(
            challenge_1_4_2::compose(challenge_1_4_1::id, plus_2).call((2,)),
            4,
        );

        assert_eq!(
            challenge_1_4_2::compose(plus_2, challenge_1_4_1::id).call_once((2,)),
            4,
        );

        assert_eq!(
            challenge_1_4_2::compose(plus_2, challenge_1_4_1::id).call_mut((2,)),
            4,
        );

        assert_eq!(
            challenge_1_4_2::compose(plus_2, challenge_1_4_1::id).call((2,)),
            4,
        );
    }
}
