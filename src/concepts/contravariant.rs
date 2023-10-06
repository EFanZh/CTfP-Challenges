use fn_traits::fns::ComposeFn;
use fn_traits::{fns, FnMut};

pub trait Contravariant<T> {
    type ContraMap<F, U>: Contravariant<U>
    where
        F: FnMut<(U,), Output = T>;

    fn contra_map<F, U>(self, f: F) -> Self::ContraMap<F, U>
    where
        F: FnMut<(U,), Output = T>;
}

impl<T, W> Contravariant<T> for W
where
    W: FnMut<(T,)>,
{
    type ContraMap<F, U> = ComposeFn<F, Self>
    where
        F: FnMut<(U,), Output = T>;

    fn contra_map<F, U>(self, f: F) -> Self::ContraMap<F, U>
    where
        F: FnMut<(U,), Output = T>,
    {
        fns::compose(f, self)
    }
}
