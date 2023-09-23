use fn_traits::FnOnce;

pub trait Product<A, B> {
    type Factorizer<T, F, G>: FnOnce<(T,), Output = Self>
    where
        F: FnOnce<(T,), Output = A>,
        G: FnOnce<(T,), Output = B>,
        T: Clone;

    fn left(self) -> A;
    fn right(self) -> B;

    fn factorizer<T, F, G>(f: F, g: G) -> Self::Factorizer<T, F, G>
    where
        F: FnOnce<(T,), Output = A>,
        G: FnOnce<(T,), Output = B>,
        T: Clone;
}

pub struct TupleFactorizer<F, G> {
    f: F,
    g: G,
}

impl<T, F, G> FnOnce<(T,)> for TupleFactorizer<F, G>
where
    F: FnOnce<(T,)>,
    G: FnOnce<(T,)>,
    T: Clone,
{
    type Output = (F::Output, G::Output);

    fn call_once(self, args: (T,)) -> Self::Output {
        (self.f.call_once(args.clone()), self.g.call_once(args))
    }
}

impl<A, B> Product<A, B> for (A, B) {
    type Factorizer<T, F, G> = TupleFactorizer<F, G>
    where
        F: FnOnce<(T,), Output = A>,
        G: FnOnce<(T,), Output = B>,
        T: Clone;

    fn left(self) -> A {
        self.0
    }

    fn right(self) -> B {
        self.1
    }

    fn factorizer<T, F, G>(f: F, g: G) -> Self::Factorizer<T, F, G>
    where
        F: FnOnce<(T,), Output = A>,
        G: FnOnce<(T,), Output = B>,
        T: Clone,
    {
        TupleFactorizer { f, g }
    }
}
