pub trait Terminal {
    fn from<T>(value: T) -> Self;
}

impl Terminal for () {
    fn from<T>(value: T) -> Self {
        drop(value);
    }
}
