#[cfg(test)]
mod tests {
    use crate::strength::ship::{
        spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions,
    };

    #[test]
    fn filled_zeros_test() {
        let test_spatium_functions = SpatiumFunctions::new(vec![
            SpatiumFunction::new(0, -62.5, -56.25, 0.0, 0.0),
            SpatiumFunction::new(1, -56.25, -50.0, 0.0, 0.0),
            SpatiumFunction::new(2, -50.0, -43.75, 0.0, 0.0),
            SpatiumFunction::new(3, -43.75, -37.5, 0.0, 0.0),
            SpatiumFunction::new(4, -37.5, -31.25, 0.0, 0.0),
            SpatiumFunction::new(5, -31.25, -25.0, 0.0, 0.0),
            SpatiumFunction::new(6, -25.0, -18.75, 0.0, 0.0),
            SpatiumFunction::new(7, -18.75, -12.5, 0.0, 0.0),
            SpatiumFunction::new(8, -12.5, -6.25, 0.0, 0.0),
            SpatiumFunction::new(9, -6.25, 0.0, 0.0, 0.0),
            SpatiumFunction::new(10, 0.0, 6.25, 0.0, 0.0),
            SpatiumFunction::new(11, 6.25, 12.5, 0.0, 0.0),
            SpatiumFunction::new(12, 12.5, 18.75, 0.0, 0.0),
            SpatiumFunction::new(13, 18.75, 25.0, 0.0, 0.0),
            SpatiumFunction::new(14, 25.0, 31.25, 0.0, 0.0),
            SpatiumFunction::new(15, 31.25, 37.5, 0.0, 0.0),
            SpatiumFunction::new(16, 37.5, 43.75, 0.0, 0.0),
            SpatiumFunction::new(17, 43.75, 50.0, 0.0, 0.0),
            SpatiumFunction::new(18, 50.0, 56.25, 0.0, 0.0),
            SpatiumFunction::new(19, 56.25, 62.5, 0.0, 0.0),
        ]);
        let spatium_functions = SpatiumFunctions::filled_zeros(20, 125.0);
        assert_eq!(test_spatium_functions, spatium_functions);
    }

    #[test]
    fn add_test() {
        let test_spatium_functions = SpatiumFunctions::new(vec![
            SpatiumFunction::new(0, -62.5, -56.25, 1.66, 1.66),
            SpatiumFunction::new(1, -56.25, -50.0, 0.0, 0.0),
            SpatiumFunction::new(2, -50.0, -43.75, 0.0, 0.0),
            SpatiumFunction::new(3, -43.75, -37.5, 0.0, 0.0),
            SpatiumFunction::new(4, -37.5, -31.25, 0.0, 0.0),
            SpatiumFunction::new(5, -31.25, -25.0, 0.0, 0.0),
            SpatiumFunction::new(6, -25.0, -18.75, 0.0, 0.0),
            SpatiumFunction::new(7, -18.75, -12.5, 0.0, 0.0),
            SpatiumFunction::new(8, -12.5, -6.25, 0.0, 0.0),
            SpatiumFunction::new(9, -6.25, 0.0, 0.0, 0.0),
            SpatiumFunction::new(10, 0.0, 6.25, 0.0, 0.0),
            SpatiumFunction::new(11, 6.25, 12.5, 0.0, 0.0),
            SpatiumFunction::new(12, 12.5, 18.75, 0.0, 0.0),
            SpatiumFunction::new(13, 18.75, 25.0, 0.0, 0.0),
            SpatiumFunction::new(14, 25.0, 31.25, 0.0, 0.0),
            SpatiumFunction::new(15, 31.25, 37.5, 0.0, 0.0),
            SpatiumFunction::new(16, 37.5, 43.75, 0.0, 0.0),
            SpatiumFunction::new(17, 43.75, 50.0, 0.0, 0.0),
            SpatiumFunction::new(18, 50.0, 56.25, 0.0, 0.0),
            SpatiumFunction::new(19, 56.25, 62.5, 0.0, 0.0),
        ]);
        let spatium_function = SpatiumFunction::new(0, -62.5, -56.25, 1.658, 1.658);
        let mut spatium_functions = SpatiumFunctions::filled_zeros(20, 125.0);
        spatium_functions.add(spatium_function);
        assert_eq!(test_spatium_functions, spatium_functions);
    }

    #[test]
    fn max_test() {
        let s_fs = SpatiumFunctions::new(vec![
            SpatiumFunction::new(0, -62.5, -56.25, 1.66, 1.66),
            SpatiumFunction::new(1, -56.25, -50.0, 0.0, 0.0),
            SpatiumFunction::new(2, -50.0, -43.75, 0.0, 0.0),
            SpatiumFunction::new(3, -43.75, -37.5, 0.0, 0.0),
            SpatiumFunction::new(4, -37.5, -31.25, 0.0, 0.0),
            SpatiumFunction::new(5, -31.25, -25.0, 0.0, 0.0),
            SpatiumFunction::new(6, -25.0, -18.75, 0.0, 0.0),
            SpatiumFunction::new(7, -18.75, -12.5, 0.0, 0.0),
            SpatiumFunction::new(8, -12.5, -6.25, 0.0, 0.0),
            SpatiumFunction::new(9, -6.25, 0.0, 0.0, 0.0),
            SpatiumFunction::new(10, 0.0, 6.25, 0.0, 0.0),
            SpatiumFunction::new(11, 6.25, 12.5, 0.0, 0.0),
            SpatiumFunction::new(12, 12.5, 18.75, 0.0, 0.0),
            SpatiumFunction::new(13, 18.75, 25.0, 0.0, 0.0),
            SpatiumFunction::new(14, 25.0, 31.25, 0.0, 0.0),
            SpatiumFunction::new(15, 31.25, 37.5, 0.0, 0.0),
            SpatiumFunction::new(16, 37.5, 43.75, 0.0, 0.0),
            SpatiumFunction::new(17, 43.75, 50.0, 0.0, 0.0),
            SpatiumFunction::new(18, 50.0, 56.25, 0.0, 0.0),
            SpatiumFunction::new(19, 56.25, 62.5, 0.0, 3.0),
        ]);
        assert_eq!(3.0, s_fs.max().unwrap());
    }

    #[test]
    fn max_with_negative_value_test() {
        let s_fs = SpatiumFunctions::new(vec![
            SpatiumFunction::new(0, -62.5, -56.25, 1.66, 1.66),
            SpatiumFunction::new(1, -56.25, -50.0, 0.0, 0.0),
            SpatiumFunction::new(2, -50.0, -43.75, 0.0, 0.0),
            SpatiumFunction::new(3, -43.75, -37.5, 0.0, 0.0),
            SpatiumFunction::new(4, -37.5, -31.25, 0.0, 0.0),
            SpatiumFunction::new(5, -31.25, -25.0, 0.0, 0.0),
            SpatiumFunction::new(6, -25.0, -18.75, 0.0, 0.0),
            SpatiumFunction::new(7, -18.75, -12.5, 0.0, 0.0),
            SpatiumFunction::new(8, -12.5, -6.25, 0.0, 0.0),
            SpatiumFunction::new(9, -6.25, 0.0, 0.0, 0.0),
            SpatiumFunction::new(10, 0.0, 6.25, 0.0, 0.0),
            SpatiumFunction::new(11, 6.25, 12.5, 0.0, 0.0),
            SpatiumFunction::new(12, 12.5, 18.75, 0.0, 0.0),
            SpatiumFunction::new(13, 18.75, 25.0, 0.0, 0.0),
            SpatiumFunction::new(14, 25.0, 31.25, 0.0, 0.0),
            SpatiumFunction::new(15, 31.25, 37.5, 0.0, 0.0),
            SpatiumFunction::new(16, 37.5, 43.75, 0.0, 0.0),
            SpatiumFunction::new(17, 43.75, 50.0, 0.0, 0.0),
            SpatiumFunction::new(18, 50.0, 56.25, 0.0, 0.0),
            SpatiumFunction::new(19, 56.25, 62.5, 0.0, -3.0),
        ]);
        assert_eq!(3.0, s_fs.max().unwrap());
    }

    #[test]
    fn max_none_test() {
        let s_fs = SpatiumFunctions::new(vec![]);
        assert_eq!(None, s_fs.max());
    }
}
