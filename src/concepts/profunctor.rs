use fn_traits::fns::ComposeFn;
use fn_traits::{fns, FnMut};

pub trait Profunctor<I, T, U> {
    type DiMap<F, G, V>: Profunctor<I, V, G::Output>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(U,)>;

    fn dimap<F, G, V>(self, f: F, g: G) -> Self::DiMap<F, G, V>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(U,)>;
}

// Function.

pub struct FunctionProfunctor;

impl<T, U, P> Profunctor<FunctionProfunctor, T, U> for P
where
    P: FnMut<(T,), Output = U>,
{
    type DiMap<F, G, V> = ComposeFn<ComposeFn<F, Self>, G>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(U,)>;

    fn dimap<F, G, V>(self, f: F, g: G) -> Self::DiMap<F, G, V>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(U,)>,
    {
        fns::compose(fns::compose(f, self), g)
    }
}
