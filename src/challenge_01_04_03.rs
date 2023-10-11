#[cfg(test)]
mod tests {
    use crate::{challenge_01_04_01, challenge_01_04_02};
    use fn_traits::{Fn, FnMut, FnOnce};

    #[test]
    fn test_compose_identity() {
        let plus_2 = |x: u32| x + 2;

        assert_eq!(
            challenge_01_04_02::compose(challenge_01_04_01::id, plus_2).call_once((2,)),
            4,
        );

        assert_eq!(
            challenge_01_04_02::compose(challenge_01_04_01::id, plus_2).call_mut((2,)),
            4,
        );

        assert_eq!(
            challenge_01_04_02::compose(challenge_01_04_01::id, plus_2).call((2,)),
            4,
        );

        assert_eq!(
            challenge_01_04_02::compose(plus_2, challenge_01_04_01::id).call_once((2,)),
            4,
        );

        assert_eq!(
            challenge_01_04_02::compose(plus_2, challenge_01_04_01::id).call_mut((2,)),
            4,
        );

        assert_eq!(
            challenge_01_04_02::compose(plus_2, challenge_01_04_01::id).call((2,)),
            4,
        );
    }
}
