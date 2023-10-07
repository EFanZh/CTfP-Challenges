use crate::concepts::bifunctor::Bifunctor;
use fn_traits::FnMut;

pub enum PreList<T, U> {
    Nil,
    Cons(T, U),
}

impl<T, U> Bifunctor<T, U> for PreList<T, U> {
    type FMap<F, G> = PreList<F::Output, G::Output>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>;

    fn bimap<F, G>(self, mut f: F, mut g: G) -> Self::FMap<F, G>
    where
        F: FnMut<(T,)>,
        G: FnMut<(U,)>,
    {
        match self {
            PreList::Nil => PreList::Nil,
            PreList::Cons(head, tail) => PreList::Cons(f.call_mut((head,)), g.call_mut((tail,))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreList;
    use crate::concepts::bifunctor::Bifunctor;
    use fn_traits::{fns, Fn};
    use std::convert;

    #[test]
    fn test_pre_list_preservation_of_composition() {
        let f_1 = |x: u8| u16::from(x + 2);
        let f_2 = |x: u16| u32::from(x * 3);
        let g_1 = |x: u64| u128::from(x + 5);
        let g_2 = |x: u128| x * 7;

        let compose_then_map =
            |x| PreList::bimap(x, fns::compose(f_1, f_2), fns::compose(g_1, g_2));

        let map_then_compose = |x| {
            fns::compose(
                |x| PreList::bimap(x, f_1, g_1),
                |x| PreList::bimap(x, f_2, g_2),
            )
            .call((x,))
        };

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
        let id = |x| PreList::bimap(x, convert::identity, convert::identity);

        assert!(matches!(id(PreList::Nil), PreList::Nil));
        assert!(matches!(id(PreList::Cons(2, 3)), PreList::Cons(2, 3)));
    }
}
