use fn_traits::{FnMut, FnOnce};

pub trait Functor {
    type Map<'a, T>
    where
        T: 'a;

    type FMap<'a, T, F>: FnOnce<(Self::Map<'a, T>,)>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a;

    fn fmap<'a, T, F>(&mut self, f: F) -> Self::FMap<'a, T, F>
    where
        F: FnMut<(T,)> + 'a;
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
    type Map<'a, T> = Option<T>
    where
        T: 'a;

    type FMap<'a, T, F> = OptionFMap<F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a;

    fn fmap<'a, T, F>(&mut self, f: F) -> Self::FMap<'a, T, F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a,
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
    type Map<'a, T> = Vec<T>
    where
        T: 'a;

    type FMap<'a, T, F> = VecFMap<F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a;

    fn fmap<'a, T, F>(&mut self, f: F) -> Self::FMap<'a, T, F>
    where
        T: 'a,
        F: FnMut<(T,)> + 'a,
    {
        Self::FMap { f }
    }
}
