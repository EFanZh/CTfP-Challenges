pub enum Either<A, B> {
    Left(A),
    Right(B),
}

#[cfg(test)]
mod tests {
    use super::Either;

    #[test]
    fn test_either() {
        assert!(matches!(Either::Left::<u32, &str>(4), Either::Left(4)));

        assert!(matches!(
            Either::Right::<u32, &str>("xyz"),
            Either::Right("xyz"),
        ));
    }
}
