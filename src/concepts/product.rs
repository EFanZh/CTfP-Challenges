use fn_traits::FnOnce;

pub trait Product<I, A, B> {
    fn left(self) -> A;
    fn right(self) -> B;

    fn factorize<F, G>(self, f: F, g: G) -> (A, B)
    where
        F: FnOnce<(Self,), Output = A>,
        G: FnOnce<(Self,), Output = B>,
        Self: Clone;
}

// Tuple.

pub struct TupleProduct;

impl<A, B> Product<TupleProduct, A, B> for (A, B) {
    fn left(self) -> A {
        self.0
    }

    fn right(self) -> B {
        self.1
    }

    fn factorize<F, G>(self, f: F, g: G) -> (A, B)
    where
        F: FnOnce<(Self,), Output = A>,
        G: FnOnce<(Self,), Output = B>,
        Self: Clone,
    {
        (f.call_once((self.clone(),)), g.call_once((self,)))
    }
}
