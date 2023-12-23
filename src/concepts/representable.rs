use crate::concepts::functor::Functor;
use fn_traits::FnMut;

pub trait Representable<C1, C2, T>: Functor<C2, T> {
    type Rep;
    type IndexOutput: FnMut<(Self::Rep,), Output = T>;

    fn tabulate<F>(f: F) -> Self
    where
        F: FnMut<(Self::Rep,), Output = T>;

    fn index(self) -> Self::IndexOutput;
}
