use crate::concepts::functor::Functor;
use crate::concepts::representable::Representable;
use fn_traits::FnMut;

pub struct Pair<T> {
    pub left: T,
    pub right: T,
}

pub struct PairFmap<F> {
    f: F,
}

impl<'a, T, F> fn_traits::FnOnce<(Pair<T>,)> for PairFmap<F>
where
    T: 'a,
    F: FnMut<(T,)> + 'a,
{
    type Output = Pair<F::Output>;

    fn call_once(mut self, args: (Pair<T>,)) -> Self::Output {
        Self::Output {
            left: self.f.call_mut((args.0.left,)),
            right: self.f.call_mut((args.0.right,)),
        }
    }
}

pub struct PairFunctor;

impl Functor for PairFunctor {
    type Map<'a, T> = Pair< T>
    where
        T: 'a;

    type FMap<'a, T, F> = PairFmap< F>
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

pub struct PairIndex<T> {
    value: Pair<T>,
}

impl<'a, T> fn_traits::FnOnce<(bool,)> for PairIndex<T>
where
    T: 'a,
{
    type Output = T;

    fn call_once(self, args: (bool,)) -> Self::Output {
        if args.0 {
            self.value.right
        } else {
            self.value.left
        }
    }
}

impl Representable for PairFunctor {
    type Rep = bool;

    type Index<'a, T> = PairIndex<T>
    where
        T: 'a;

    fn tabulate<'a, F>(&mut self, mut f: F) -> Self::Map<'a, F::Output>
    where
        F: FnMut<(Self::Rep,)> + 'a,
    {
        Self::Map {
            left: f.call_mut((false,)),
            right: f.call_mut((true,)),
        }
    }

    fn index<'a, T>(&mut self, value: Self::Map<'a, T>) -> Self::Index<'a, T>
    where
        T: 'a,
    {
        Self::Index { value }
    }
}

#[cfg(test)]
mod tests {
    use super::{Pair, PairFunctor};
    use crate::concepts::representable::Representable;
    use fn_traits::FnOnce;

    #[test]
    fn test_pair_representable() {
        assert!(matches!(
            PairFunctor.tabulate(|x| if x { 33 } else { 22 }),
            Pair {
                left: 22,
                right: 33
            }
        ));

        assert_eq!(
            PairFunctor
                .index(Pair { left: 2, right: 3 })
                .call_once((false,)),
            2,
        );

        assert_eq!(
            PairFunctor
                .index(Pair { left: 2, right: 3 })
                .call_once((true,)),
            3,
        );
    }
}
