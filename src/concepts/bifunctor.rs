use fn_traits::FnMut;

pub trait Bifunctor<T, U> {
    type FMap<F, G>: Bifunctor<F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(self, f: F, g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(T,)>,

        G: FnMut<(U,)>;
}

impl<T, E> Bifunctor<T, E> for Result<T, E> {
    type FMap<F, G> = Result<F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(E,)>;

    fn bimap<F, G>(self, mut f: F, mut g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(E,)>,
    {
        match self {
            Ok(value) => Ok(f.call_mut((value,))),
            Err(error) => Err(g.call_mut((error,))),
        }
    }
}
