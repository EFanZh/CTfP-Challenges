use crate::concepts::bifunctor::Bifunctor;
use fn_traits::FnMut;

impl<T, U> Bifunctor<T, U> for (T, U) {
    type FMap<F, G> = (F::Output, G::Output)
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(self, mut f: F, mut g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>,
    {
        (f.call_mut((self.0,)), g.call_mut((self.1,)))
    }
}
