pub fn all_false(_: bool) -> bool {
    false
}

pub fn all_true(_: bool) -> bool {
    true
}

pub fn identity(x: bool) -> bool {
    x
}

pub fn invert(x: bool) -> bool {
    !x
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all_false() {
        assert!(!super::all_false(false));
        assert!(!super::all_false(true));
    }

    #[test]
    fn test_all_true() {
        assert!(super::all_true(false));
        assert!(super::all_true(true));
    }

    #[test]
    fn test_identity() {
        assert!(!super::identity(false));
        assert!(super::identity(true));
    }

    #[test]
    fn test_invert() {
        assert!(super::invert(false));
        assert!(!super::invert(true));
    }
}
