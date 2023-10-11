use fn_traits::FnMut;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Memoize<F, Args>
where
    F: FnMut<Args> + ?Sized,
{
    cache: HashMap<Args, F::Output>,
    f: F,
}

impl<F, Args> FnMut<Args> for Memoize<F, Args>
where
    F: FnMut<Args> + ?Sized,
    F::Output: Clone,
    Args: Clone + Eq + Hash,
{
    type Output = F::Output;

    fn call_mut(&mut self, args: Args) -> Self::Output {
        if let Some(value) = self.cache.get(&args) {
            value.clone()
        } else {
            let output = self.f.call_mut(args.clone());

            self.cache.insert(args, output.clone());

            output
        }
    }
}

pub fn memoize<F, Args>(f: F) -> Memoize<F, Args>
where
    F: FnMut<Args>,
    F::Output: Clone,
    Args: Clone + Eq + Hash,
{
    Memoize {
        cache: HashMap::new(),
        f,
    }
}

#[cfg(test)]
mod tests {
    use fn_traits::FnMut;
    use std::cell::Cell;

    #[test]
    fn test_memoize() {
        let call_count = Cell::new(0_u32);

        let mut f = super::memoize(|x| {
            call_count.set(call_count.get() + 1);

            x * 2
        });

        assert_eq!(call_count.get(), 0);

        assert_eq!(f.call_mut((1,)), 2);
        assert_eq!(call_count.get(), 1);

        assert_eq!(f.call_mut((2,)), 4);
        assert_eq!(call_count.get(), 2);

        assert_eq!(f.call_mut((3,)), 6);
        assert_eq!(call_count.get(), 3);

        assert_eq!(f.call_mut((1,)), 2);
        assert_eq!(call_count.get(), 3);

        assert_eq!(f.call_mut((2,)), 4);
        assert_eq!(call_count.get(), 3);

        assert_eq!(f.call_mut((3,)), 6);
        assert_eq!(call_count.get(), 3);
    }
}
