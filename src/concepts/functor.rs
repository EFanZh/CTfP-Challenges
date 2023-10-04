use fn_traits::FnMut;

pub trait Functor<T> {
    type FMap<F>: Functor<F::Output>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(self, f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>;
}

impl<T> Functor<T> for Option<T> {
    type FMap<F> = Option<F::Output>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(self, mut f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>,
    {
        self.map(|value| f.call_mut((value,)))
    }
}

impl<T> Functor<T> for Vec<T> {
    type FMap<F> = Vec<F::Output>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(self, mut f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>,
    {
        self.into_iter().map(|value| f.call_mut((value,))).collect()
    }
}
