use fn_traits::fns::ComposeFn;
use fn_traits::{fns, FnMut, FnOnce};
use std::marker::PhantomData;

pub trait Contravariant<C, T>: Sized {
    type ContraMapOutput<U, F>: Contravariant<C, U>
    where
        F: FnMut<(U,), Output = T>;

    type ContraMap<U, F>: FnOnce<(Self,), Output = Self::ContraMapOutput<U, F>>
    where
        F: FnMut<(U,), Output = T>;

    fn contra_map<F, U>(self, f: F) -> Self::ContraMap<U, F>
    where
        F: FnMut<(U,), Output = T>;
}

pub struct OpFunctor;

pub struct OpContraMap<F, T> {
    f: F,
    _phantom: PhantomData<T>,
}

impl<W, F, T> FnOnce<(W,)> for OpContraMap<F, T>
where
    W: FnMut<(F::Output,)>,
    F: FnMut<(T,)>,
{
    type Output = ComposeFn<F, W>;

    fn call_once(self, args: (W,)) -> Self::Output {
        fns::compose(self.f, args.0)
    }
}

impl<T, W> Contravariant<OpFunctor, T> for W
where
    W: FnMut<(T,)>,
{
    type ContraMapOutput<U, F> = ComposeFn<F, Self>
    where
        F: FnMut<(U,), Output = T>;

    type ContraMap<U, F> = OpContraMap<F, U>
    where
        F: FnMut<(U,), Output = T>;

    fn contra_map<F, U>(self, f: F) -> Self::ContraMap<U, F>
    where
        F: FnMut<(U,), Output = T>,
    {
        OpContraMap {
            f,
            _phantom: PhantomData,
        }
    }
}
