use crate::concepts::bifunctor::Bifunctor;
use fn_traits::FnMut;
use std::marker::PhantomData;

pub struct K2<C, A, B>(pub C, pub PhantomData<(A, B)>);

pub struct K2BiFunctor;

pub struct K2BiMap<F, G> {
    _f: F,
    _g: G,
}

impl<C, A, B, F, G> FnMut<(K2<C, A, B>,)> for K2BiMap<F, G>
where
    F: FnMut<(A,)>,
    G: FnMut<(B,)>,
{
    type Output = K2<C, F::Output, G::Output>;

    fn call_mut(&mut self, args: (K2<C, A, B>,)) -> Self::Output {
        K2(args.0 .0, PhantomData)
    }
}

impl<C, A, B> Bifunctor<K2BiFunctor, A, B> for K2<C, A, B> {
    type BiMapOutput<F, G> = K2<C, F::Output, G::Output>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    type BiMap<F, G> = K2BiMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>,
    {
        K2BiMap { _f: f, _g: g }
    }
}

pub struct Fst<A, B>(pub A, pub PhantomData<B>);

pub struct FstBiFunctor;

pub struct FstBiMap<F, G> {
    f: F,
    _g: G,
}

impl<A, B, F, G> FnMut<(Fst<A, B>,)> for FstBiMap<F, G>
where
    F: FnMut<(A,)>,
    G: FnMut<(B,)>,
{
    type Output = Fst<F::Output, G::Output>;

    fn call_mut(&mut self, args: (Fst<A, B>,)) -> Self::Output {
        Fst(self.f.call_mut((args.0 .0,)), PhantomData)
    }
}

impl<A, B> Bifunctor<FstBiFunctor, A, B> for Fst<A, B> {
    type BiMapOutput<F, G> = Fst<F::Output, G::Output>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    type BiMap<F, G> = FstBiMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>,
    {
        FstBiMap { f, _g: g }
    }
}

pub struct Snd<A, B>(pub B, pub PhantomData<A>);

pub struct SndBiFunctor;

pub struct SndBiMap<F, G> {
    _f: F,
    g: G,
}

impl<A, B, F, G> FnMut<(Snd<A, B>,)> for SndBiMap<F, G>
where
    F: FnMut<(A,)>,
    G: FnMut<(B,)>,
{
    type Output = Snd<F::Output, G::Output>;

    fn call_mut(&mut self, args: (Snd<A, B>,)) -> Self::Output {
        Snd(self.g.call_mut((args.0 .0,)), PhantomData)
    }
}

impl<A, B> Bifunctor<SndBiFunctor, A, B> for Snd<A, B> {
    type BiMapOutput<F, G> = Snd<F::Output, G::Output>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    type BiMap<F, G> = SndBiMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(A,)>,
        G: FnMut<(B,)>,
    {
        SndBiMap { _f: f, g }
    }
}

#[cfg(test)]
mod tests {
    use super::{Fst, Snd, K2};
    use crate::concepts::bifunctor::Bifunctor;
    use fn_traits::{fns, FnMut};
    use std::convert;
    use std::marker::PhantomData;

    type MyK2<T, U> = K2<u32, T, U>;

    #[test]
    fn test_k2_preservation_of_composition() {
        let f_1 = |x: u8| u16::from(x + 2);
        let f_2 = |x: u16| u32::from(x * 3);
        let g_1 = |x: u64| u128::from(x + 5);
        let g_2 = |x: u128| x * 7;

        let compose_then_map =
            |x| MyK2::bimap(fns::compose(f_1, f_2), fns::compose(g_1, g_2)).call_mut((x,));

        let map_then_compose =
            |x| fns::compose(MyK2::bimap(f_1, g_1), MyK2::bimap(f_2, g_2)).call_mut((x,));

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
        let id = |x| MyK2::bimap(convert::identity, convert::identity).call_mut((x,));

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

        let compose_then_map =
            |x| Fst::bimap(fns::compose(f_1, f_2), fns::compose(g_1, g_2)).call_mut((x,));

        let map_then_compose =
            |x| fns::compose(Fst::bimap(f_1, g_1), Fst::bimap(f_2, g_2)).call_mut((x,));

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
        let id = |x| Fst::bimap(convert::identity, convert::identity).call_mut((x,));

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

        let compose_then_map =
            |x| Snd::bimap(fns::compose(f_1, f_2), fns::compose(g_1, g_2)).call_mut((x,));

        let map_then_compose =
            |x| fns::compose(Snd::bimap(f_1, g_1), Snd::bimap(f_2, g_2)).call_mut((x,));

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
        let id = |x| Snd::bimap(convert::identity, convert::identity).call_mut((x,));

        assert!(matches!(
            id(Snd::<u32, _>(7, PhantomData)),
            Snd(7, PhantomData),
        ));
    }
}
