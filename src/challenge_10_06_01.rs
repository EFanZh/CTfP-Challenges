use crate::concepts::functor::Functor;
use fn_traits::{FnMut, FnOnce};

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

pub struct ListFunctor;

pub struct ListFMap<F> {
    f: F,
}

impl<T, F> FnOnce<(List<T>,)> for ListFMap<F>
where
    F: FnMut<(T,)>,
{
    type Output = List<F::Output>;

    fn call_once(mut self, args: (List<T>,)) -> Self::Output {
        match args.0 {
            List::Nil => List::Nil,
            List::Cons(head, tail) => {
                List::Cons(self.f.call_mut((head,)), Box::new(self.call_once((*tail,))))
            }
        }
    }
}

impl Functor for ListFunctor {
    type Map<T> = List<T>;

    type FMap<T, F> = ListFMap<F>
    where
        F: FnMut<(T,)>;

    fn fmap<T, F>(&mut self, f: F) -> Self::FMap<T, F>
    where
        F: FnMut<(T,)>,
    {
        Self::FMap { f }
    }
}

pub fn option_to_list<T>(value: Option<T>) -> List<T> {
    match value {
        None => List::Nil,
        Some(value) => List::Cons(value, Box::new(List::Nil)),
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    use crate::challenge_10_06_01::ListFunctor;
    use crate::concepts::functor::{Functor, OptionFunctor};
    use fn_traits::{fns, FnOnce};

    #[test]
    fn test_naturality() {
        let f = |x| x + 2;

        let fmap_then_transform =
            |x| fns::compose(OptionFunctor.fmap(f), super::option_to_list).call_once((x,));

        let transform_then_fmap =
            |x| fns::compose(super::option_to_list, ListFunctor.fmap(f)).call_once((x,));

        assert!(matches!(fmap_then_transform(None), List::Nil));
        assert!(matches!(transform_then_fmap(None), List::Nil));

        assert!(matches!(fmap_then_transform(Some(3)), List::Cons(5, _)));
        assert!(matches!(transform_then_fmap(Some(3)), List::Cons(5, _)));
    }
}
