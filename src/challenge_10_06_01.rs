use crate::concepts::functor::Functor;
use fn_traits::FnMut;

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

pub struct ListFunctor;

pub struct ListFMap<F> {
    f: F,
}

impl<T, F> FnMut<(List<T>,)> for ListFMap<F>
where
    F: FnMut<(T,)>,
{
    type Output = List<F::Output>;

    fn call_mut(&mut self, args: (List<T>,)) -> Self::Output {
        match args.0 {
            List::Nil => List::Nil,
            List::Cons(head, tail) => {
                List::Cons(self.f.call_mut((head,)), Box::new(self.call_mut((*tail,))))
            }
        }
    }
}

impl<T> Functor<ListFunctor, T> for List<T> {
    type FMapOutput<F> = List<F::Output>
    where
        F: FnMut<(T,)>;

    type FMap<F> = ListFMap<F>
    where
        F: FnMut<(T,)>;

    fn fmap<F>(f: F) -> Self::FMap<F>
    where
        F: FnMut<(T,)>,
    {
        ListFMap { f }
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
    use crate::concepts::functor::Functor;
    use fn_traits::{fns, FnMut};

    #[test]
    fn test_naturality() {
        let f = |x| x + 2;

        let fmap_then_transform =
            |x| fns::compose(Option::fmap(f), super::option_to_list).call_mut((x,));

        let transform_then_fmap =
            |x| fns::compose(super::option_to_list, List::fmap(f)).call_mut((x,));

        assert!(matches!(fmap_then_transform(None), List::Nil));
        assert!(matches!(transform_then_fmap(None), List::Nil));

        assert!(matches!(fmap_then_transform(Some(3)), List::Cons(5, _)));
        assert!(matches!(transform_then_fmap(Some(3)), List::Cons(5, _)));
    }
}
