pub trait Semigroup {
    fn multiply(self, rhs: Self) -> Self;
}

impl Semigroup for String {
    fn multiply(self, rhs: Self) -> Self {
        self + rhs.as_str()
    }
}

pub trait Monoid: Semigroup {
    fn unit() -> Self;
}

impl Monoid for String {
    fn unit() -> Self {
        Self::new()
    }
}
