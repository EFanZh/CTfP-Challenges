use crate::challenge_05_08_04::Either;

pub fn left(n: i32) -> (bool, i32) {
    (false, n)
}

pub fn right(b: bool) -> (bool, i32) {
    (true, i32::from(b))
}

pub fn m1(x: (bool, i32)) -> Either<i32, bool> {
    if x.0 {
        match x.1 {
            0 => Either::Right(false),
            1 => Either::Right(true),
            _ => Either::Left(77), // <-- See here.
        }
    } else {
        Either::Left(x.1)
    }
}

pub fn m2(x: (bool, i32)) -> Either<i32, bool> {
    if x.0 {
        match x.1 {
            0 => Either::Right(false),
            1 => Either::Right(true),
            _ => Either::Left(88), // <-- See here.
        }
    } else {
        Either::Left(x.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge_05_08_04;

    type Either = challenge_05_08_04::Either<i32, bool>;

    #[test]
    fn test_m1() {
        assert!(matches!(super::m1(super::left(2)), Either::Left(2)));
        assert!(matches!(super::m1(super::left(3)), Either::Left(3)));

        assert!(matches!(
            super::m1(super::right(false)),
            Either::Right(false)
        ));

        assert!(matches!(super::m1(super::right(true)), Either::Right(true)));
    }

    #[test]
    fn test_m2() {
        assert!(matches!(super::m2(super::left(2)), Either::Left(2)));
        assert!(matches!(super::m2(super::left(3)), Either::Left(3)));

        assert!(matches!(
            super::m2(super::right(false)),
            Either::Right(false)
        ));

        assert!(matches!(super::m2(super::right(true)), Either::Right(true)));
    }
}
