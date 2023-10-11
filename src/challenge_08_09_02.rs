use crate::challenge_05_08_04::Either;
use std::marker::PhantomData;

pub struct Const<T, U>(pub T, pub PhantomData<U>);
pub struct Identity<T>(pub T);

pub fn option_to_either<T>(value: Option<T>) -> Either<Const<(), T>, Identity<T>> {
    match value {
        None => Either::Left(Const((), PhantomData)),
        Some(value) => Either::Right(Identity(value)),
    }
}

pub fn either_to_option<T>(value: Either<Const<(), T>, Identity<T>>) -> Option<T> {
    match value {
        Either::Left(Const((), PhantomData)) => None,
        Either::Right(Identity(value)) => Some(value),
    }
}

#[cfg(test)]
mod tests {
    use super::{Const, Identity};
    use crate::challenge_05_08_04::Either;
    use fn_traits::{fns, Fn};
    use std::marker::PhantomData;

    #[test]
    fn test_isomorphism() {
        let id_option = fns::compose(super::option_to_either, super::either_to_option);

        assert_eq!(id_option.call((None::<u32>,)), None);
        assert_eq!(id_option.call((Some(7),)), Some(7));

        let id_either = fns::compose(super::either_to_option, super::option_to_either);

        assert!(matches!(
            id_either.call((Either::Left(Const((), PhantomData::<u32>)),)),
            Either::Left(Const((), PhantomData)),
        ));

        assert!(matches!(
            id_either.call((Either::Right(Identity(7)),)),
            Either::Right(Identity(7)),
        ));
    }
}
