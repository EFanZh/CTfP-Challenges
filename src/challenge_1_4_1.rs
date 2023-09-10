pub fn id<T>(input: T) -> T {
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_identity() {
        assert_eq!(super::id(2), 2);
        assert_eq!(super::id("foo"), "foo");
    }
}
