use fn_traits::FnMut;

pub trait Bifunctor<I, T, U>: Sized {
    type BiMap<F, G>: Bifunctor<I, F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(self, f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;
}

// Result.

pub struct ResultBifunctor;

impl<T, E> Bifunctor<ResultBifunctor, T, E> for Result<T, E> {
    type BiMap<F, G> = Result<F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(E,)>;

    fn bimap<F, G>(self, mut f: F, mut g: G) -> Self::BiMap<F, G>
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
