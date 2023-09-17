use crate::challenge_5_8_4::Either;
use fn_traits::FnOnce;
use std::convert::Infallible;

pub trait Semigroup {
    fn multiply(self, rhs: Self) -> Self;
}

impl Semigroup for String {
    fn multiply(self, rhs: Self) -> Self {
        self + rhs.as_str()
    }
}

pub trait Monoid: Semigroup {
    fn unit() -> Self;
}

impl Monoid for String {
    fn unit() -> Self {
        Self::new()
    }
}

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

pub trait Initial {
    fn into<T>(self) -> T;
}

impl Initial for Infallible {
    fn into<T>(self) -> T {
        match self {}
    }
}

pub trait Terminal {
    fn from<T>(value: T) -> Self;
}

impl Terminal for () {
    fn from<T>(value: T) -> Self {
        drop(value);
    }
}

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
