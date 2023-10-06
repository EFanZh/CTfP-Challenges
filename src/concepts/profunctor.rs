use fn_traits::fns::{self, ComposeFn};
use fn_traits::FnMut;

pub trait Profunctor<T, U> {
    type DiMap<F, G, V>: Profunctor<V, G::Output>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(U,)>;

    fn dimap<F, G, V>(self, f: F, g: G) -> Self::DiMap<F, G, V>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(U,)>;
}

impl<T, P> Profunctor<T, P::Output> for P
where
    P: FnMut<(T,)>,
{
    type DiMap<F, G, V> = ComposeFn<ComposeFn<F, Self>, G>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(P::Output,)>;

    fn dimap<F, G, V>(self, f: F, g: G) -> Self::DiMap<F, G, V>
    where
        F: FnMut<(V,), Output = T>,
        G: FnMut<(P::Output,)>,
    {
        fns::compose(fns::compose(f, self), g)
    }
}
