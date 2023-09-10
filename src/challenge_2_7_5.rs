pub fn no(_: bool) -> bool {
    false
}

pub fn yes(_: bool) -> bool {
    true
}

pub fn id(x: bool) -> bool {
    x
}

pub fn not(x: bool) -> bool {
    !x
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all_false() {
        assert!(!super::no(false));
        assert!(!super::no(true));
    }

    #[test]
    fn test_all_true() {
        assert!(super::yes(false));
        assert!(super::yes(true));
    }

    #[test]
    fn test_identity() {
        assert!(!super::id(false));
        assert!(super::id(true));
    }

    #[test]
    fn test_invert() {
        assert!(super::not(false));
        assert!(!super::not(true));
    }
}
