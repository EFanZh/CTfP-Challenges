use fn_traits::FnMut;

pub fn bimap<F, G, T, U>(
    mut f: F,
    mut g: G,
) -> impl FnMut<((T, U),), Output = (F::Output, G::Output)>
where
    F: FnMut<(T,)>,
    G: FnMut<(U,)>,
{
    move |(left, right)| (f.call_mut((left,)), g.call_mut((right,)))
}

pub fn first<F, T, U>(mut f: F) -> impl FnMut<((T, U),), Output = (F::Output, U)>
where
    F: FnMut<(T,)>,
{
    move |(left, right)| (f.call_mut((left,)), right)
}

pub fn second<F, T, U>(mut f: F) -> impl FnMut<((T, U),), Output = (T, F::Output)>
where
    F: FnMut<(U,)>,
{
    move |(left, right)| (left, f.call_mut((right,)))
}

#[cfg(test)]
mod tests {
    use fn_traits::FnMut;

    #[test]
    fn test_bimap() {
        assert_eq!(
            super::bimap(|x| x + 2, |x| x * 3).call_mut(((2, 3),)),
            (4, 9),
        );
    }

    #[test]
    fn test_first() {
        assert_eq!(super::first(|x| x + 2).call_mut(((2, 3),)), (4, 3));
    }

    #[test]
    fn test_second() {
        assert_eq!(super::second(|x| x * 3).call_mut(((2, 3),)), (2, 9));
    }
}
