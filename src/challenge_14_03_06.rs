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

impl<'a, T> Functor<'a, PairFunctor, T> for Pair<T>
where
    T: 'a,
{
    type Map<F> = Pair<F::Output>
    where
        F: FnMut<(T,)> + 'a;

    fn map<F>(self, mut f: F) -> Self::Map<F>
    where
        F: FnMut<(T,)> + 'a,
    {
        Pair {
            left: f.call_mut((self.left,)),
            right: f.call_mut((self.right,)),
        }
    }
}

impl<'a, T> Representable<'a, PairFunctor, T> for Pair<T>
where
    T: 'a,
{
    type Rep = bool;

    fn tabulate<F>(mut f: F) -> Self
    where
        F: FnMut<(Self::Rep,), Output = T> + 'a,
    {
        Self {
            left: f.call_mut((false,)),
            right: f.call_mut((true,)),
        }
    }

    fn index(self, rep: Self::Rep) -> T {
        if rep {
            self.right
        } else {
            self.left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pair;
    use crate::concepts::representable::Representable;

    #[test]
    fn test_pair_representable() {
        assert!(matches!(
            Pair::tabulate(|x| if x { 33 } else { 22 }),
            Pair {
                left: 22,
                right: 33
            }
        ));

        assert_eq!(Pair { left: 2, right: 3 }.index(false), 2);
        assert_eq!(Pair { left: 2, right: 3 }.index(true), 3);
    }
}
