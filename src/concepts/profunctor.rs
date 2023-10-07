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
