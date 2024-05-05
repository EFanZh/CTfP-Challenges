use crate::concepts::functor::Functor;
use fn_traits::FnMut;

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

pub struct ListFunctor;

impl<'a, T> Functor<'a, ListFunctor, T> for List<T>
where
    T: 'a,
{
    type Map<F> = List<F::Output>
    where
        F: FnMut<(T,)> + 'a;

    fn map<F>(self, mut f: F) -> Self::Map<F>
    where
        F: FnMut<(T,)> + 'a,
    {
        fn helper<T, F>(list: List<T>, f: &mut F) -> List<F::Output>
        where
            F: FnMut<(T,)>,
        {
            match list {
                List::Nil => List::Nil,
                List::Cons(head, tail) => {
                    List::Cons(f.call_mut((head,)), Box::new(helper(*tail, f)))
                }
            }
        }

        helper(self, &mut f)
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
            |x| fns::compose(|x| Functor::map(x, f), super::option_to_list).call_mut((x,));

        let transform_then_fmap =
            |x| fns::compose(super::option_to_list, |x| List::map(x, f)).call_mut((x,));

        assert!(matches!(fmap_then_transform(None), List::Nil));
        assert!(matches!(transform_then_fmap(None), List::Nil));

        assert!(matches!(fmap_then_transform(Some(3)), List::Cons(5, _)));
        assert!(matches!(transform_then_fmap(Some(3)), List::Cons(5, _)));
    }
}
