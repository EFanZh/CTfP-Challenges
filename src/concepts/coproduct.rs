use fn_traits::FnOnce;

pub trait Coproduct<I, A, B> {
    fn left(value: A) -> Self;
    fn right(value: B) -> Self;

    fn factorize<F, G>(self, f: F, g: G) -> F::Output
    where
        Self: Sized,
        F: FnOnce<(A,)>,
        G: FnOnce<(B,), Output = F::Output>;
}

// Result.

pub struct ResultCoproduct;

impl<A, B> Coproduct<ResultCoproduct, A, B> for Result<A, B> {
    fn left(value: A) -> Self {
        Ok(value)
    }

    fn right(value: B) -> Self {
        Err(value)
    }

    fn factorize<F, G>(self, f: F, g: G) -> F::Output
    where
        Self: Sized,
        F: FnOnce<(A,)>,
        G: FnOnce<(B,), Output = F::Output>,
    {
        match self {
            Ok(value) => f.call_once((value,)),
            Err(error) => g.call_once((error,)),
        }
    }
}
