use crate::concepts::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

impl Monoid for String {
    fn empty() -> Self {
        Self::new()
    }
}
