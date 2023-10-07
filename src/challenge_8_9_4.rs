use crate::concepts::bifunctor::Bifunctor;
use fn_traits::FnMut;
use std::marker::PhantomData;

pub struct K2<C, A, B>(pub C, PhantomData<(A, B)>);

impl<C, A, B> Bifunctor<A, B> for K2<C, A, B> {
    type FMap<F, G> = K2<C, F::Output, G::Output>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    fn bimap<F, G>(self, _f: F, _g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>,
    {
        K2(self.0, PhantomData)
    }
}

pub struct Fst<A, B>(pub A, PhantomData<B>);

impl<A, B> Bifunctor<A, B> for Fst<A, B> {
    type FMap<F, G> = Fst<F::Output, G::Output>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    fn bimap<F, G>(self, mut f: F, _g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>,
    {
        Fst(f.call_mut((self.0,)), PhantomData)
    }
}

pub struct Snd<A, B>(pub B, PhantomData<A>);

impl<A, B> Bifunctor<A, B> for Snd<A, B> {
    type FMap<F, G> = Snd<F::Output, G::Output>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    fn bimap<F, G>(self, _f: F, mut g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>,
    {
        Snd(g.call_mut((self.0,)), PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::{Fst, Snd, K2};
    use crate::concepts::bifunctor::Bifunctor;
    use fn_traits::{fns, Fn};
    use std::convert;
    use std::marker::PhantomData;

    #[test]
    fn test_k2_preservation_of_composition() {
        let f_1 = |x: u8| u16::from(x + 2);
        let f_2 = |x: u16| u32::from(x * 3);
        let g_1 = |x: u64| u128::from(x + 5);
        let g_2 = |x: u128| x * 7;

        let compose_then_map = |x| K2::bimap(x, fns::compose(f_1, f_2), fns::compose(g_1, g_2));

        let map_then_compose =
            |x| fns::compose(|x| K2::bimap(x, f_1, g_1), |x| K2::bimap(x, f_2, g_2)).call((x,));

        assert!(matches!(
            compose_then_map(K2(7, PhantomData)),
            K2(7, PhantomData)
        ));
        assert!(matches!(
            map_then_compose(K2(7, PhantomData)),
            K2(7, PhantomData)
        ));
    }

    #[test]
    fn test_k2_preservation_of_identity() {
        let id = |x| K2::bimap(x, convert::identity, convert::identity);

        assert!(matches!(
            id(K2::<_, u32, u64>(7, PhantomData)),
            K2(7, PhantomData),
        ));
    }

    #[test]
    fn test_fst_preservation_of_composition() {
        let f_1 = |x: u8| u16::from(x + 2);
        let f_2 = |x: u16| u32::from(x * 3);
        let g_1 = |x: u64| u128::from(x + 5);
        let g_2 = |x: u128| x * 7;

        let compose_then_map = |x| Fst::bimap(x, fns::compose(f_1, f_2), fns::compose(g_1, g_2));

        let map_then_compose =
            |x| fns::compose(|x| Fst::bimap(x, f_1, g_1), |x| Fst::bimap(x, f_2, g_2)).call((x,));

        assert!(matches!(
            compose_then_map(Fst(7, PhantomData)),
            Fst(27, PhantomData),
        ));

        assert!(matches!(
            map_then_compose(Fst(7, PhantomData)),
            Fst(27, PhantomData),
        ));
    }

    #[test]
    fn test_fst_preservation_of_identity() {
        let id = |x| Fst::bimap(x, convert::identity, convert::identity);

        assert!(matches!(
            id(Fst::<_, u32>(7, PhantomData)),
            Fst(7, PhantomData),
        ));
    }

    #[test]
    fn test_snd_preservation_of_composition() {
        let f_1 = |x: u8| u16::from(x + 2);
        let f_2 = |x: u16| u32::from(x * 3);
        let g_1 = |x: u64| u128::from(x + 5);
        let g_2 = |x: u128| x * 7;

        let compose_then_map = |x| Snd::bimap(x, fns::compose(f_1, f_2), fns::compose(g_1, g_2));

        let map_then_compose =
            |x| fns::compose(|x| Snd::bimap(x, f_1, g_1), |x| Snd::bimap(x, f_2, g_2)).call((x,));

        assert!(matches!(
            compose_then_map(Snd(7, PhantomData)),
            Snd(84, PhantomData),
        ));

        assert!(matches!(
            map_then_compose(Snd(7, PhantomData)),
            Snd(84, PhantomData),
        ));
    }

    #[test]
    fn test_snd_preservation_of_identity() {
        let id = |x| Snd::bimap(x, convert::identity, convert::identity);

        assert!(matches!(
            id(Snd::<u32, _>(7, PhantomData)),
            Snd(7, PhantomData),
        ));
    }
}
