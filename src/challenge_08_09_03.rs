use crate::concepts::bifunctor::Bifunctor;
use fn_traits::FnMut;

pub enum PreList<T, U> {
    Nil,
    Cons(T, U),
}

pub struct PreListBifunctor;

pub struct PreListBiMap<F, G> {
    f: F,
    g: G,
}

impl<T, U, F, G> FnMut<(PreList<T, U>,)> for PreListBiMap<F, G>
where
    F: FnMut<(T,)>,
    G: FnMut<(U,)>,
{
    type Output = PreList<F::Output, G::Output>;

    fn call_mut(&mut self, args: (PreList<T, U>,)) -> Self::Output {
        match args.0 {
            PreList::Nil => PreList::Nil,
            PreList::Cons(head, tail) => {
                PreList::Cons(self.f.call_mut((head,)), self.g.call_mut((tail,)))
            }
        }
    }
}

impl<T, U> Bifunctor<PreListBifunctor, T, U> for PreList<T, U> {
    type BiMapOutput<F, G> = PreList<F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    type BiMap<F, G> = PreListBiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(f: F, g: G) -> Self::BiMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>,
    {
        PreListBiMap { f, g }
    }
}

#[cfg(test)]
mod tests {
    use super::PreList;
    use crate::concepts::bifunctor::Bifunctor;
    use fn_traits::{fns, FnMut};
    use std::convert;

    #[test]
    fn test_pre_list_preservation_of_composition() {
        let f_1 = |x: u8| u16::from(x + 2);
        let f_2 = |x: u16| u32::from(x * 3);
        let g_1 = |x: u64| u128::from(x + 5);
        let g_2 = |x: u128| x * 7;

        let compose_then_map =
            |x| PreList::bimap(fns::compose(f_1, f_2), fns::compose(g_1, g_2)).call_mut((x,));

        let map_then_compose =
            |x| fns::compose(PreList::bimap(f_1, g_1), PreList::bimap(f_2, g_2)).call_mut((x,));

        assert!(matches!(compose_then_map(PreList::Nil), PreList::Nil));
        assert!(matches!(map_then_compose(PreList::Nil), PreList::Nil));

        assert!(matches!(
            compose_then_map(PreList::Cons(2, 3)),
            PreList::Cons(12, 56)
        ));

        assert!(matches!(
            map_then_compose(PreList::Cons(2, 3)),
            PreList::Cons(12, 56)
        ));
    }

    #[test]
    fn test_pre_list_preservation_of_identity() {
        let id = |x| PreList::bimap(convert::identity, convert::identity).call_mut((x,));

        assert!(matches!(id(PreList::Nil), PreList::Nil));
        assert!(matches!(id(PreList::Cons(2, 3)), PreList::Cons(2, 3)));
    }
}
