#[cfg(test)]
mod tests {
    use crate::core::linear_interpolation::LinearInterpolation;


    #[test]
    fn interpolated_value_ok_test() {
        let interpolation = LinearInterpolation::new(5.0, 1.0, 2.0, 6.0);
        assert_eq!(Ok(4.0), interpolation.interpolated_value(3.0));
        assert_eq!(Ok(3.0), interpolation.interpolated_value(4.0));
        assert_eq!(Ok(2.0), interpolation.interpolated_value(5.0));
        assert_eq!(Ok(1.0), interpolation.interpolated_value(6.0));
    }

    #[test]
    fn interpolated_value_error_test() {
        let interpolation = LinearInterpolation::new(5.0, 1.0, 2.0, 6.0);
        assert_eq!(Err("Function argument 'x' should be x_0 < x < x_1.".to_owned()), interpolation.interpolated_value(1.0));
        assert_eq!(Err("Function argument 'x' should be x_0 < x < x_1.".to_owned()), interpolation.interpolated_value(7.0));
    }
}