use crate::concepts::profunctor::Profunctor;
use fn_traits::fns::{self, ComposeFn, ConvertIdentityFn};
use fn_traits::FnMut;
use std::collections::HashMap;
use std::hash::Hash;

pub struct FunctorialHashMap<K, V, KF>
where
    KF: FnMut<(K,)>,
{
    data: HashMap<KF::Output, V>,
    key_fn: KF,
}

impl<K, V> FunctorialHashMap<K, V, ConvertIdentityFn> {
    pub fn new(data: HashMap<K, V>) -> Self {
        Self {
            data,
            key_fn: ConvertIdentityFn::default(),
        }
    }
}

impl<K, V, KF> FunctorialHashMap<K, V, KF>
where
    KF: FnMut<(K,)>,
    KF::Output: Eq + Hash,
{
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.data.get_mut(&self.key_fn.call_mut((key,)))
    }
}

pub struct FunctorialHashMapProfunctor;

impl<K, V, KF> Profunctor<FunctorialHashMapProfunctor, K, V> for FunctorialHashMap<K, V, KF>
where
    KF: FnMut<(K,)>,
    KF::Output: Eq + Hash,
{
    type DiMap<F, G, U> = FunctorialHashMap<U, G::Output, ComposeFn<F, KF>>
    where
        F: FnMut<(U,), Output = K>,
        G: FnMut<(V,)>;

    fn dimap<F, G, U>(self, f: F, mut g: G) -> Self::DiMap<F, G, U>
    where
        F: FnMut<(U,), Output = K>,
        G: FnMut<(V,)>,
    {
        FunctorialHashMap {
            data: self
                .data
                .into_iter()
                .map(|(key, value)| (key, g.call_mut((value,))))
                .collect(),
            key_fn: fns::compose(f, self.key_fn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctorialHashMap;
    use crate::concepts::profunctor::Profunctor;
    use std::collections::HashMap;

    #[test]
    fn test_functorial_hash_map() {
        let mut map = FunctorialHashMap::new(HashMap::from([(2, 3), (5, 7)]));

        assert_eq!(map.get_mut(2).copied(), Some(3));
        assert_eq!(map.get_mut(5).copied(), Some(7));

        // Map into a new type.

        let mut new_map = map.dimap(|x| x + 2, |x| x * 3);

        // Test composition.

        assert_eq!(new_map.get_mut(0).copied(), Some(9));
        assert_eq!(new_map.get_mut(3).copied(), Some(21));
    }
}
