use fn_traits::fns::ComposeFn;
use fn_traits::{fns, FnMut};
use std::marker::PhantomData;

pub trait Contravariant<I, T> {
    type ContraMap<U, F>: Contravariant<I, U>
    where
        F: FnMut<(U,), Output = T>;

    fn contra_map<U, F>(self, f: F) -> Self::ContraMap<U, F>
    where
        F: FnMut<(U,), Output = T>;
}

// Op.

pub struct OpContravariant<A> {
    _phantom: PhantomData<A>,
}

impl<A, T, W> Contravariant<OpContravariant<A>, T> for W
where
    W: FnMut<(T,), Output = A>,
{
    type ContraMap<U, F> = ComposeFn<F, Self>
    where
        F: FnMut<(U,), Output = T>;

    fn contra_map<U, F>(self, f: F) -> Self::ContraMap<U, F>
    where
        F: FnMut<(U,), Output = T>,
    {
        fns::compose(f, self)
    }
}
