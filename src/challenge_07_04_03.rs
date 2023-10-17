use crate::concepts::functor::Functor;
use fn_traits::fns::ComposeFn;
use fn_traits::{fns, FnMut, FnOnce};
use std::marker::PhantomData;

pub trait Reader<T, U>: FnMut<(T,), Output = U> {}

impl<T, U, F> Reader<T, U> for F where F: FnMut<(T,), Output = U> {}

pub struct ReaderFunctor<T> {
    inner: PhantomData<T>,
}

pub struct ReaderFMap<T, F> {
    f: F,
    _phantom: PhantomData<T>,
}

impl<T, R, F> FnOnce<(R,)> for ReaderFMap<T, F>
where
    R: FnMut<(T,)>,
    F: FnMut<(R::Output,)>,
{
    type Output = ComposeFn<R, F>;

    fn call_once(self, args: (R,)) -> Self::Output {
        fns::compose(args.0, self.f)
    }
}

impl<T, U, R> Functor<ReaderFunctor<T>, U> for R
where
    R: FnMut<(T,), Output = U>,
{
    type FMapOutput<F> = ComposeFn<R, F>
    where
        F: FnMut<(U,)>;

    type FMap<F> = ReaderFMap<T, F>
    where
        F: FnMut<(U,)>;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: FnMut<(U,)>,
    {
        ReaderFMap {
            f,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Reader, ReaderFunctor};
    use crate::concepts::functor::Functor;
    use fn_traits::{fns, FnMut, FnOnce};
    use std::convert;

    fn fmap<F, R, T, U>(
        f: F,
    ) -> impl FnOnce<(R,), Output = <R as Functor<ReaderFunctor<T>, U>>::FMapOutput<F>>
    where
        R: Reader<T, U>,
        F: FnMut<(U,)>,
    {
        R::fmap(f)
    }

    #[test]
    fn test_reader() {
        let reader = |x: u8| u16::from(x + 1);

        // Preservation of identity.

        assert_eq!(
            fmap(convert::identity).call_once((reader,)).call_mut((2,)),
            3,
        );

        assert_eq!(reader(2), 3);

        assert_eq!(
            fmap(convert::identity).call_once((reader,)).call_mut((3,)),
            4,
        );

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
