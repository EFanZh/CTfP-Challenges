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
        fn make_rest<'a, F>(
            mut index: usize,
            mut f: F,
        ) -> Box<dyn FnOnce() -> Stream<'a, F::Output> + 'a>
        where
            F: FnMut<(usize,)> + 'a,
        {
            Box::new(move || {
                index += 1;

                Stream {
                    first: f.call_mut((index,)),
                    rest: make_rest(index, f),
                }
            })
        }

        Self::Map {
            first: f.call_mut((0,)),
            rest: make_rest(0, f),
        }
    }

    fn index<'a, T>(&mut self, value: Self::Map<'a, T>) -> Self::Index<'a, T>
    where
        T: 'a,
    {
        Self::Index { value }
    }
}

pub fn square_stream() -> Stream<'static, usize> {
    StreamFunctor.tabulate(|x| x * x)
}

#[cfg(test)]
mod tests {
    use super::Stream;

    #[test]
    fn test_square_stream() {
        let mut stream = Some(super::square_stream());

        let mut next = || {
            let Stream { first, rest } = stream.take().unwrap();

            stream = Some(rest());

            first
        };

        assert_eq!(next(), 0);
        assert_eq!(next(), 1);
        assert_eq!(next(), 4);
        assert_eq!(next(), 9);
        assert_eq!(next(), 16);
        assert_eq!(next(), 25);
    }
}
