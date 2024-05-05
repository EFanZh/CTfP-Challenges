use fn_traits::fns::ComposeFn;
use fn_traits::{fns, FnMut};
use std::marker::PhantomData;

pub trait Functor<'a, I, T>
where
    T: 'a,
{
    type Map<F>: Functor<'a, I, F::Output>
    where
        F: FnMut<(T,)> + 'a;

    fn map<F>(self, f: F) -> Self::Map<F>
    where
        F: FnMut<(T,)> + 'a;
}

// Option.

pub struct OptionFunctor;

impl<'a, T> Functor<'a, OptionFunctor, T> for Option<T>
where
    T: 'a,
{
    type Map< F> = Option<F::Output>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a;

    fn map<F>(self, mut f: F) -> Self::Map<F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a,
    {
        self.map(|value| f.call_mut((value,)))
    }
}

// Reader.

pub struct ReaderFunctor<A> {
    _phantom: PhantomData<A>,
}

impl<'a, A, U, R> Functor<'a, ReaderFunctor<A>, U> for R
where
    U: 'a,
    R: FnMut<(A,), Output = U>,
{
    type Map<F> = ComposeFn<Self, F>
    where
        F: FnMut<(U,)> + 'a;

    fn map<F>(self, f: F) -> Self::Map<F>
    where
        F: FnMut<(U,)> + 'a,
    {
        fns::compose(self, f)
    }
}
