#[cfg(test)]
mod tests {
    use crate::concepts::contravariant::{Contravariant, OpContravariant};
    use fn_traits::{fns, FnMut, FnOnce};

    fn contra_map<F, W, T>(
        f: F,
    ) -> impl FnOnce<
        (W,),
        Output = <W as Contravariant<OpContravariant<W::Output>, F::Output>>::ContraMap<T, F>,
    >
    where
        F: FnMut<(T,)>,
        W: FnMut<(F::Output,)>,
    {
        move |x| W::contra_map(x, f)
    }

    #[test]
    fn test_bool_op_to_result_op() {
        fn bool_op_to_result_op<T>(
            mut f: impl FnMut<(T,), Output = bool>,
        ) -> impl FnMut<(T,), Output = Result<(), ()>> {
            move |x| if f.call_mut((x,)) { Ok(()) } else { Err(()) }
        }

        let f = |x: &str| x.parse::<i32>().unwrap();

        let contra_map_then_transform =
            |x| fns::compose(contra_map(f), bool_op_to_result_op).call_once((x,));

        let transform_then_contra_map =
            |x| fns::compose(bool_op_to_result_op, contra_map(f)).call_once((x,));

        let op = |x: i32| x > 0;
        let transformed_1 = |x| contra_map_then_transform(op).call_mut((x,));
        let transformed_2 = |x| transform_then_contra_map(op).call_mut((x,));

        assert_eq!(transformed_1("-2"), Err(()));
        assert_eq!(transformed_2("-2"), Err(()));

        assert_eq!(transformed_1("0"), Err(()));
        assert_eq!(transformed_2("0"), Err(()));

        assert_eq!(transformed_1("3"), Ok(()));
        assert_eq!(transformed_2("3"), Ok(()));
    }
}
