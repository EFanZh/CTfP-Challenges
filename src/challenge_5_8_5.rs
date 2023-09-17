use crate::challenge_5_8_4::Either;

pub fn i(n: i32) -> i32 {
    n
}

pub fn j(b: bool) -> i32 {
    if b {
        0
    } else {
        1
    }
}

pub fn m(e: Either<i32, bool>) -> i32 {
    match e {
        Either::Left(n) => i(n),
        Either::Right(b) => j(b),
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge_5_8_4;

    #[test]
    fn test_m() {
        type Either = challenge_5_8_4::Either<i32, bool>;

        assert_eq!(super::i(2), super::m(Either::Left(2)));
        assert_eq!(super::i(3), super::m(Either::Left(3)));
        assert_eq!(super::j(true), super::m(Either::Right(true)));
        assert_eq!(super::j(false), super::m(Either::Right(false)));
    }
}
