use crate::concepts::functor::Functor;
use crate::concepts::representable::Representable;
use fn_traits::{fns, FnMut};

pub struct Stream<'a, T> {
    pub first: T,
    pub rest: Box<dyn FnOnce() -> Self + 'a>,
}

pub struct StreamFmap<F> {
    f: F,
}

impl<'a, T, F> fn_traits::FnOnce<(Stream<'a, T>,)> for StreamFmap<F>
where
    T: 'a,
    F: FnMut<(T,)> + 'a,
{
    type Output = Stream<'a, F::Output>;

    fn call_once(mut self, args: (Stream<'a, T>,)) -> Self::Output {
        Self::Output {
            first: self.f.call_mut((args.0.first,)),
            rest: Box::new(|| fns::compose(args.0.rest, self).call_once(())),
        }
    }
}

pub struct StreamFunctor;

impl Functor for StreamFunctor {
    type Map<'a, T> = Stream<'a, T>
    where
        T: 'a;

    type FMap<'a, T, F> = StreamFmap< F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a;

    fn fmap<'a, T, F>(&mut self, f: F) -> Self::FMap<'a, T, F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a,
    {
        Self::FMap { f }
    }
}

pub struct StreamIndex<'a, T> {
    value: Stream<'a, T>,
}

impl<'a, T> fn_traits::FnOnce<(usize,)> for StreamIndex<'a, T>
where
    T: 'a,
{
    type Output = T;

    fn call_once(self, args: (usize,)) -> Self::Output {
        if args.0 == 0 {
            self.value.first
        } else {
            StreamFunctor
                .index((self.value.rest)())
                .call_once((args.0 - 1,))
        }
    }
}

impl Representable for StreamFunctor {
    type Rep = usize;

    type Index<'a, T> = StreamIndex<'a, T>
    where
        T: 'a;

    fn tabulate<'a, F>(&mut self, mut f: F) -> Self::Map<'a, F::Output>
    where
        F: FnMut<(Self::Rep,)> + 'a,
    {
        Self::Map {
            first: f.call_mut((0,)),
            rest: Box::new(|| Self.tabulate(fns::compose(|x| x + 1, f))),
        }
    }

    fn index<'a, T>(&mut self, value: Self::Map<'a, T>) -> Self::Index<'a, T>
    where
        T: 'a,
    {
        Self::Index { value }
    }
}
