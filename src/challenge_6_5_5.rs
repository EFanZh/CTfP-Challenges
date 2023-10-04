use crate::challenge_5_8_4::Either;

pub fn sum_to_product<T>(value: Either<T, T>) -> (bool, T) {
    match value {
        Either::Left(value) => (false, value),
        Either::Right(value) => (true, value),
    }
}

pub fn product_to_sum<T>(value: (bool, T)) -> Either<T, T> {
    if value.0 {
        Either::Right(value.1)
    } else {
        Either::Left(value.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge_5_8_4::Either;
    use fn_traits::{fns, Fn};

    #[test]
    fn test_isomorphism() {
        let id_sum = fns::compose(super::sum_to_product, super::product_to_sum);

        assert!(matches!(id_sum.call((Either::Left(2),)), Either::Left(2)));
        assert!(matches!(id_sum.call((Either::Right(3),)), Either::Right(3)));

        let id_product = fns::compose(super::product_to_sum, super::sum_to_product);

        assert_eq!(id_product.call(((false, 2),)), (false, 2));
        assert_eq!(id_product.call(((true, 3),)), (true, 3));
    }
}
