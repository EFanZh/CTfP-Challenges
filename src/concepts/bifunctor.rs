use fn_traits::FnMut;

pub trait Bifunctor<C, T, U>: Sized {
    type BiMapOutput<F, G>: Bifunctor<C, F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    type BiMap<F, G>: FnMut<(Self,), Output = Self::BiMapOutput<F, G>>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;
}

pub struct ResultBifunctor;

pub struct ResultBiMap<F, G> {
    f: F,
    g: G,
}

impl<T, U, F, G> FnMut<(Result<T, U>,)> for ResultBiMap<F, G>
where
    F: FnMut<(T,)>,
    G: FnMut<(U,)>,
{
    type Output = Result<F::Output, G::Output>;

    fn call_mut(&mut self, args: (Result<T, U>,)) -> Self::Output {
        match args.0 {
            Ok(value) => Ok(self.f.call_mut((value,))),
            Err(error) => Err(self.g.call_mut((error,))),
        }
    }
}

impl<T, E> Bifunctor<ResultBifunctor, T, E> for Result<T, E> {
    type BiMapOutput<F, G> = Result<F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(E,)>;

    type BiMap<F, G> = ResultBiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(E,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(E,)>,
    {
        ResultBiMap { f, g }
    }
}
