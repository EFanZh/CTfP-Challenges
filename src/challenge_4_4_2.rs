pub fn safe_reciprocal(x: f64) -> Option<f64> {
    if x == 0.0 {
        None
    } else {
        Some(x.recip())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_safe_reciprocal() {
        assert_eq!(super::safe_reciprocal(0.0), None);
        assert_eq!(super::safe_reciprocal(0.5), Some(2.0));
        assert_eq!(super::safe_reciprocal(1.0), Some(1.0));
        assert_eq!(super::safe_reciprocal(2.0), Some(0.5));
    }
}
