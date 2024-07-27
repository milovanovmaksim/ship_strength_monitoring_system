#[cfg(test)]
mod tests {
    use crate::core::linear_interpolation::LinearInterpolation;

    use std::{env, sync::Once};

    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug"); // off / error / warn / info / debug / trace
                                               // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
            let _ = env_logger::try_init();
        })
    }

    #[test]
    fn interpolated_value_ok_test() {
        let interpolation = LinearInterpolation::new(5.0, 1.0, 2.0, 6.0);
        assert_eq!(Ok(5.0), interpolation.interpolated_value(2.0));
        assert_eq!(Ok(4.0), interpolation.interpolated_value(3.0));
        assert_eq!(Ok(3.0), interpolation.interpolated_value(4.0));
        assert_eq!(Ok(2.0), interpolation.interpolated_value(5.0));
        assert_eq!(Ok(1.0), interpolation.interpolated_value(6.0));
    }

    #[test]
    fn interpolated_value_error_test() {
        call_once();
        let interpolation = LinearInterpolation::new(5.0, 1.0, 2.0, 6.0);
        assert_eq!(
            Err("Значение аргумента 'x' функции 'interpolated_value' должно лежать в диапазоне 'x_0 <= x <= x_1'.".to_owned()),
            interpolation.interpolated_value(1.0)
        );
        assert_eq!(
            Err("Значение аргумента 'x' функции 'interpolated_value' должно лежать в диапазоне 'x_0 <= x <= x_1'.".to_owned()),
            interpolation.interpolated_value(7.0)
        );
        let interpolation = LinearInterpolation::new(5.0, 1.0, 2.0, 2.0);
        assert_eq!(
            Err("x_0 и x_1 не должны быть равны между собой".to_owned()),
            interpolation.interpolated_value(1.0)
        );
        let interpolation = LinearInterpolation::new(5.0, 1.0, 3.0, 1.0);
        assert_eq!(
            Err("Значение аргумента x_0 должно быть меньше x_1.".to_owned()),
            interpolation.interpolated_value(2.0)
        );
    }
}
