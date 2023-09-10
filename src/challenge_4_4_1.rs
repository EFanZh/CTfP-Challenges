use fn_traits::{Fn, FnMut, FnOnce};

pub struct Compose<G, F>
where
    F: ?Sized,
{
    g: G,
    f: F,
}

impl<T, G, F, U, V> FnOnce<(T,)> for Compose<G, F>
where
    G: FnOnce<(U,), Output = Option<V>>,
    F: FnOnce<(T,), Output = Option<U>>,
{
    type Output = G::Output;

    fn call_once(self, args: (T,)) -> Self::Output {
        self.g.call_once((self.f.call_once(args)?,))
    }
}

impl<T, G, F, U, V> FnMut<(T,)> for Compose<G, F>
where
    G: FnMut<(U,), Output = Option<V>>,
    F: FnMut<(T,), Output = Option<U>> + ?Sized,
{
    type Output = G::Output;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.g.call_mut((self.f.call_mut(args)?,))
    }
}

impl<T, G, F, U, V> Fn<(T,)> for Compose<G, F>
where
    G: Fn<(U,), Output = Option<V>>,
    F: Fn<(T,), Output = Option<U>> + ?Sized,
{
    type Output = G::Output;

    fn call(&self, args: (T,)) -> Self::Output {
        self.g.call((self.f.call(args)?,))
    }
}

pub fn compose<G, F>(g: G, f: F) -> Compose<G, F> {
    Compose { g, f }
}

pub fn id<T>(x: T) -> Option<T> {
    Some(x)
}

#[cfg(test)]
mod tests {
    use fn_traits::{Fn, FnMut, FnOnce};

    #[test]
    fn test_compose() {
        let mut none_none = super::compose(|_: u32| None::<u32>, |_| None);
        let mut none_some = super::compose(|_| None::<u32>, |x| Some(x + 2));
        let mut some_none = super::compose(|x: u32| Some(x * 3), |_| None);
        let mut some_some = super::compose(|x| Some(x * 3), |x| Some(x + 2));

        assert_eq!(none_none.call((7,)), None);
        assert_eq!(none_none.call_mut((7,)), None);
        assert_eq!(none_none.call_once((7,)), None);

        assert_eq!(none_some.call((7,)), None);
        assert_eq!(none_some.call_mut((7,)), None);
        assert_eq!(none_some.call_once((7,)), None);

        assert_eq!(some_none.call((7,)), None);
        assert_eq!(some_none.call_mut((7,)), None);
        assert_eq!(some_none.call_once((7,)), None);

        assert_eq!(some_some.call((7,)), Some(27));
        assert_eq!(some_some.call_mut((7,)), Some(27));
        assert_eq!(some_some.call_once((7,)), Some(27));
    }

    #[test]
    fn test_identity() {
        let f = super::compose(super::id, |x| Some(x + 3));
        let g = super::compose(|x| Some(x + 3), super::id);

        assert_eq!(f.call((7,)), Some(10));
        assert_eq!(g.call((7,)), Some(10));
    }
}
