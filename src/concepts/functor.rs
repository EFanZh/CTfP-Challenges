use fn_traits::{FnMut, FnOnce};

pub trait Functor<C, T>: Sized {
    type FMapOutput<F>: Functor<C, F::Output>
    where
        F: FnMut<(T,)>;

    type FMap<F>: FnOnce<(Self,), Output = Self::FMapOutput<F>>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>;
}

pub struct OptionFunctor;

pub struct OptionFMap<F> {
    f: F,
}

impl<T, F> FnOnce<(Option<T>,)> for OptionFMap<F>
where
    F: FnMut<(T,)>,
{
    type Output = Option<F::Output>;

    fn call_once(mut self, args: (Option<T>,)) -> Self::Output {
        args.0.map(|value| self.f.call_mut((value,)))
    }
}

impl<T> Functor<OptionFunctor, T> for Option<T> {
    type FMapOutput<F> = Option<F::Output>
    where
        F: FnMut<(T,)>;

    type FMap<F> = OptionFMap<F>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>,
    {
        OptionFMap { f }
    }
}

pub struct VecFunctor;

pub struct VecFMap<F> {
    f: F,
}

impl<T, F> FnOnce<(Vec<T>,)> for VecFMap<F>
where
    F: FnMut<(T,)>,
{
    type Output = Vec<F::Output>;

    fn call_once(mut self, args: (Vec<T>,)) -> Self::Output {
        args.0
            .into_iter()
            .map(|value| self.f.call_mut((value,)))
            .collect()
    }
}

impl<T> Functor<VecFunctor, T> for Vec<T> {
    type FMapOutput<F> = Vec<F::Output>
    where
        F: FnMut<(T,)>;

    type FMap<F> = VecFMap<F>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>,
    {
        VecFMap { f }
    }
}
