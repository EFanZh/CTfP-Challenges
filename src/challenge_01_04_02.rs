use fn_traits::{Fn, FnMut, FnOnce};

pub struct Compose<G, F>
where
    F: ?Sized,
{
    g: G,
    f: F,
}

impl<T, G, F> FnOnce<(T,)> for Compose<G, F>
where
    G: FnOnce<(F::Output,)>,
    F: FnOnce<(T,)>,
{
    type Output = G::Output;

    fn call_once(self, args: (T,)) -> Self::Output {
        self.g.call_once((self.f.call_once(args),))
    }
}

impl<T, G, F> FnMut<(T,)> for Compose<G, F>
where
    G: FnMut<(F::Output,)>,
    F: FnMut<(T,)> + ?Sized,
{
    type Output = G::Output;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.g.call_mut((self.f.call_mut(args),))
    }
}

impl<T, G, F> Fn<(T,)> for Compose<G, F>
where
    G: Fn<(F::Output,)>,
    F: Fn<(T,)> + ?Sized,
{
    type Output = G::Output;

    fn call(&self, args: (T,)) -> Self::Output {
        self.g.call((self.f.call(args),))
    }
}

pub fn compose<G, F>(g: G, f: F) -> Compose<G, F> {
    Compose { g, f }
}

#[cfg(test)]
mod tests {
    use fn_traits::{Fn, FnMut, FnOnce};

    #[test]
    fn test_compose() {
        let plus_2 = |x: u32| x + 2;
        let mul_3 = |x: u32| x * 3;

        assert_eq!(super::compose(plus_2, mul_3).call_once((2,)), 8);
        assert_eq!(super::compose(plus_2, mul_3).call_mut((2,)), 8);
        assert_eq!(super::compose(plus_2, mul_3).call((2,)), 8);

        assert_eq!(super::compose(mul_3, plus_2).call_once((2,)), 12);
        assert_eq!(super::compose(mul_3, plus_2).call_mut((2,)), 12);
        assert_eq!(super::compose(mul_3, plus_2).call((2,)), 12);
    }
}
