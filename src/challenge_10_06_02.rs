use crate::challenge_07_04_03::Reader;
use crate::challenge_10_06_01::List;

pub fn reader_to_list_1<T>(_reader: impl Reader<(), T>) -> List<T> {
    List::Nil
}

pub fn reader_to_list_2<T>(mut reader: impl Reader<(), T>) -> List<T> {
    List::Cons(reader.call_mut(((),)), Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use crate::challenge_10_06_01::List;

    #[test]
    fn test_reader_to_list_1() {
        assert!(matches!(super::reader_to_list_1(|()| 0), List::Nil));
        assert!(matches!(super::reader_to_list_1(|()| 1), List::Nil));
    }

    #[test]
    fn test_reader_to_list_2() {
        assert!(matches!(super::reader_to_list_2(|()| 0), List::Cons(0, _)));
        assert!(matches!(super::reader_to_list_2(|()| 1), List::Cons(1, _)));
    }
}
