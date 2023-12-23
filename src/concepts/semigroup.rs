pub trait Semigroup {
    fn append(self, rhs: Self) -> Self;
}

impl Semigroup for String {
    fn append(self, rhs: Self) -> Self {
        self + rhs.as_str()
    }
}
