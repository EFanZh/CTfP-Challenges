use crate::challenge_05_08_04::Either;

pub fn option_to_either<T>(value: Option<T>) -> Either<(), T> {
    match value {
        None => Either::Left(()),
        Some(value) => Either::Right(value),
    }
}

pub fn either_to_option<T>(value: Either<(), T>) -> Option<T> {
    match value {
        Either::Left(()) => None,
        Either::Right(value) => Some(value),
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge_05_08_04::Either;
    use fn_traits::{fns, FnOnce};

    #[test]
    fn test_isomorphism() {
        let id_1 = fns::compose(super::option_to_either, super::either_to_option);

        assert_eq!(id_1.call_once((None::<u32>,)), None);
        assert_eq!(id_1.call_once((Some(7),)), Some(7));

        let id_2 = fns::compose(super::either_to_option, super::option_to_either);

        assert!(matches!(
            id_2.call_once((Either::<(), u32>::Left(()),)),
            Either::Left(()),
        ));

        assert!(matches!(
            id_2.call_once((Either::<(), u32>::Right(7),)),
            Either::Right(7),
        ));
    }
}
