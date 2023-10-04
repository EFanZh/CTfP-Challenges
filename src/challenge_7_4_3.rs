use fn_traits::FnMut;

pub trait Reader<T, U>: FnMut<(T,), Output = U> {
    type FMap<G>: Reader<T, G::Output>
    where
        G: FnMut<(U,)>;

    fn fmap<F>(self, f: F) -> Self::FMap<F>
    where
        F: FnMut<(U,)>;
}

pub struct FMap<F, G> {
    f: F,
    g: G,
}

impl<T, F, G> FnMut<(T,)> for FMap<F, G>
where
    F: FnMut<(T,)>,
    G: FnMut<(F::Output,)>,
{
    type Output = G::Output;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.g.call_mut((self.f.call_mut(args),))
    }
}

impl<T, F> Reader<T, F::Output> for F
where
    F: FnMut<(T,)>,
{
    type FMap<G> = FMap<Self, G>
    where
        G: FnMut<(F::Output,)>;

    fn fmap<G>(self, g: G) -> Self::FMap<G>
    where
        G: FnMut<(F::Output,)>,
    {
        FMap { f: self, g }
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;
    use fn_traits::{fns, FnMut, FnOnce};
    use std::convert;

    fn fmap<F, R, U, T>(f: F) -> impl FnOnce<(R,), Output = R::FMap<F>>
    where
        F: FnMut<(U,)>,
        R: Reader<T, U>,
    {
        move |reader: R| reader.fmap(f)
    }

    #[test]
    fn test_reader() {
        let reader = |x: u8| u16::from(x + 1);

        // Preservation of identity.

        assert_eq!(reader.fmap(convert::identity).call_mut((2,)), 3);
        assert_eq!(reader(2), 3);

        assert_eq!(reader.fmap(convert::identity).call_mut((3,)), 4);
        assert_eq!(reader(3), 4);

        // Compose.

        let f = |x| u32::from(x * 2);
        let g = |x| u64::from(x + 7);

        let mut compose_then_map = fmap(fns::compose(f, g)).call_once((reader,));
        let mut map_then_compose = fns::compose(fmap(f), fmap(g)).call_once((reader,));

        assert_eq!(compose_then_map.call_mut((2,)), 13);
        assert_eq!(map_then_compose.call_mut((2,)), 13);

        assert_eq!(compose_then_map.call_mut((3,)), 15);
        assert_eq!(map_then_compose.call_mut((3,)), 15);
    }
}
