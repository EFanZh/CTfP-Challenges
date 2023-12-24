use crate::concepts::functor::Functor;
use fn_traits::{FnMut, FnOnce};

pub trait Representable: Functor {
    type Rep;

    type Index<'a, T>: FnOnce<(Self::Rep,), Output = T>
    where
        T: 'a;

    fn tabulate<'a, F>(&mut self, f: F) -> Self::Map<'a, F::Output>
    where
        F: FnMut<(Self::Rep,)> + 'a;

    fn index<'a, T>(&mut self, value: Self::Map<'a, T>) -> Self::Index<'a, T>
    where
        T: 'a;
}
