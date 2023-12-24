use fn_traits::{FnMut, FnOnce};

pub trait Functor {
    type Map<T>;

    type FMap<T, F>: FnOnce<(Self::Map<T>,)>
    where
        F: FnMut<(T,)>;

    fn fmap<T, F>(&mut self, f: F) -> Self::FMap<T, F>
    where
        F: FnMut<(T,)>;
}

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

pub struct OptionFunctor;

impl Functor for OptionFunctor {
    type Map<T> = Option<T>;

    type FMap<T, F> = OptionFMap<F>
    where
        F: FnMut<(T,)>;

    fn fmap<T, F>(&mut self, f: F) -> Self::FMap<T, F>
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

impl Functor for VecFunctor {
    type Map<T> = Vec<T>;

    type FMap<T, F> = VecFMap<F>
    where
        F: FnMut<(T,)>;

    fn fmap<T, F>(&mut self, f: F) -> Self::FMap<T, F>
    where
        F: FnMut<(T,)>,
    {
        Self::FMap { f }
    }
}
