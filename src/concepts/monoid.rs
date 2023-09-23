use crate::concepts::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn unit() -> Self;
}

impl Monoid for String {
    fn unit() -> Self {
        Self::new()
    }
}
