use crate::concepts::bifunctor::Bifunctor;
use fn_traits::FnMut;

pub struct TupleBifunctor;

pub struct TupleBiMap<F, G> {
    f: F,
    g: G,
}

impl<T, U, F, G> FnMut<((T, U),)> for TupleBiMap<F, G>
where
    F: FnMut<(T,)>,
    G: FnMut<(U,)>,
{
    type Output = (F::Output, G::Output);

    fn call_mut(&mut self, args: ((T, U),)) -> Self::Output {
        (self.f.call_mut((args.0 .0,)), self.g.call_mut((args.0 .1,)))
    }
}

impl<T, U> Bifunctor<TupleBifunctor, T, U> for (T, U) {
    type BiMapOutput<F, G> = (F::Output, G::Output)
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    type BiMap<F, G> = TupleBiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>,
    {
        TupleBiMap { f, g }
    }
}
