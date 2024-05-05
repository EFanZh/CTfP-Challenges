use crate::concepts::functor::Functor;
use crate::concepts::representable::Representable;
use fn_traits::FnMut;

pub struct Stream<'a, T> {
    pub first: T,
    pub rest: Box<dyn FnOnce() -> Self + 'a>,
}

pub struct StreamFunctor;

impl<'a, T> Functor<'a, StreamFunctor, T> for Stream<'a, T>
where
    T: 'a,
{
    type Map< F> = Stream<'a, F::Output>
    where
        F: FnMut<(T,)> + 'a;

    fn map<F>(self, mut f: F) -> Self::Map<F>
    where
        F: FnMut<(T,)> + 'a,
    {
        Stream {
            first: f.call_mut((self.first,)),
            rest: Box::new(|| (self.rest)().map(f)),
        }
    }
}

impl<'a, T> Representable<'a, StreamFunctor, T> for Stream<'a, T>
where
    T: 'a,
{
    type Rep = usize;

    fn tabulate<F>(f: F) -> Self
    where
        F: FnMut<(Self::Rep,), Output = T> + 'a,
    {
        fn helper<'a, F>(mut index: usize, mut f: F) -> Stream<'a, F::Output>
        where
            F: FnMut<(usize,)> + 'a,
        {
            let first = f.call_mut((index,));

            index += 1;

            Stream {
                first,
                rest: Box::new(move || helper(index, f)),
            }
        }

        helper(0, f)
    }

    fn index(mut self, mut rep: Self::Rep) -> T {
        while rep != 0 {
            rep -= 1;
            self = (self.rest)();
        }

        self.first
    }
}

pub fn square_stream() -> Stream<'static, usize> {
    Stream::tabulate(|x| x * x)
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
