use crate::challenge_07_04_03::Reader;

pub fn reader_to_option_1<T>(_reader: impl Reader<(), T>) -> Option<T> {
    None
}

pub fn reader_to_option_2<T>(mut reader: impl Reader<(), T>) -> Option<T> {
    Some(reader.call_mut(((),)))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_reader_to_option_1() {
        assert_eq!(super::reader_to_option_1(|()| 0), None);
        assert_eq!(super::reader_to_option_1(|()| 1), None);
    }

    #[test]
    fn test_reader_to_option_2() {
        assert!(matches!(super::reader_to_option_2(|()| 0), Some(0)));
        assert!(matches!(super::reader_to_option_2(|()| 1), Some(1)));
    }
}
