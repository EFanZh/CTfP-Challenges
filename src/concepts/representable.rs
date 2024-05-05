use crate::concepts::functor::Functor;
use fn_traits::FnMut;

pub trait Representable<'a, I, T>: Functor<'a, I, T>
where
    T: 'a,
{
    type Rep;

    fn tabulate<F>(f: F) -> Self
    where
        F: FnMut<(Self::Rep,), Output = T> + 'a;

    fn index(self, rep: Self::Rep) -> T;
}
