use crate::challenge_5_8_4::Either;
use fn_traits::FnOnce;

pub trait Coproduct<A, B> {
    type Factorizer<F, G>: FnOnce<(Self,), Output = F::Output>
    where
        Self: Sized,
        F: FnOnce<(A,)>,
        G: FnOnce<(B,), Output = F::Output>;

    fn left(value: A) -> Self;
    fn right(value: B) -> Self;

    fn factorizer<F, G>(f: F, g: G) -> Self::Factorizer<F, G>
    where
        Self: Sized,
        F: FnOnce<(A,)>,
        G: FnOnce<(B,), Output = F::Output>;
}

pub struct EitherFactorizer<F, G> {
    f: F,
    g: G,
}

impl<A, B, F, G> FnOnce<(Either<A, B>,)> for EitherFactorizer<F, G>
where
    F: FnOnce<(A,)>,
    G: FnOnce<(B,), Output = F::Output>,
{
    type Output = F::Output;

    fn call_once(self, args: (Either<A, B>,)) -> Self::Output {
        match args.0 {
            Either::Left(value) => self.f.call_once((value,)),
            Either::Right(value) => self.g.call_once((value,)),
        }
    }
}

impl<A, B> Coproduct<A, B> for Either<A, B> {
    type Factorizer<F, G> = EitherFactorizer<F, G>
    where
        Self: Sized,
        F: FnOnce<(A,)>,
        G: FnOnce<(B,), Output = F::Output>;

    fn left(value: A) -> Self {
        Self::Left(value)
    }

    fn right(value: B) -> Self {
        Self::Right(value)
    }

    fn factorizer<F, G>(f: F, g: G) -> Self::Factorizer<F, G>
    where
        Self: Sized,
        F: FnOnce<(A,)>,
        G: FnOnce<(B,), Output = F::Output>,
    {
        EitherFactorizer { f, g }
    }
}
