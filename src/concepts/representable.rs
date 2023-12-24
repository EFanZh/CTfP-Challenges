use crate::concepts::functor::Functor;
use fn_traits::FnMut;

pub trait Representable: Functor {
    type Rep;
    type IndexOutput: FnMut<(Self::Rep,)>;

    fn tabulate<F>(f: F) -> Self
    where
        F: FnMut<(Self::Rep,)>;

    fn index(self) -> Self::IndexOutput;
}
