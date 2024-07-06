#[cfg(test)]
mod tests {
    use log::info;

    use crate::strength::ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
    };

    #[test]
    fn from_id_test() {
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let test_spatium_function = SpatiumFunction::new(0, -62.5, -56.25, 1.5, 1.5);
        let spatium_function = SpatiumFunction::from_id(0, &ship_dimensions, 1.5, 1.5);
        info!("{:#?}", spatium_function);
        assert_eq!(test_spatium_function, spatium_function);
    }

    #[test]
    fn add_test() {
        let spatium_function = SpatiumFunction::new(0, -62.5, -56.25, 1.5, 1.5);
        let other_spatium_function = SpatiumFunction::new(0, -62.5, -56.25, 1.5, 1.5);
        let new_spatium_function = spatium_function.add(other_spatium_function);
        let test_spatium_function = SpatiumFunction::new(0, -62.5, -56.25, 3.0, 3.0);
        assert_eq!(test_spatium_function, new_spatium_function);
    }

    #[test]
    fn integral_test() {
        let spatium_function = SpatiumFunction::new(0, -62.5, -56.25, 1.5, 1.5);
        let integral = spatium_function.integral();
        let test_integral = 9.375;
        assert_eq!(test_integral, integral);
    }
}
