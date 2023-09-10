pub fn identity<T>(input: T) -> T {
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_identity() {
        assert_eq!(super::identity(2), 2);
        assert_eq!(super::identity("foo"), "foo");
    }
}
