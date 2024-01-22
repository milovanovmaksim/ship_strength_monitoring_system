#[cfg(test)]
mod tests {
    use crate::strength::ship::ship_dimensions::ShipDimensions;


    #[test]
    fn length_spatium_test() {
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_length_spatium = 6.25;
        assert_eq!(test_length_spatium, ship_dimensions.length_spatium());
    }

    #[test]
    fn coordinate_bow_test() {
        let length_between_perpendiculars = 125.0;
        let ship_dimensions = ShipDimensions::new(length_between_perpendiculars, 20, 0.6);
        let test_coordinate_bow = 62.5;
        assert_eq!(test_coordinate_bow, ship_dimensions.coordinate_bow());

    }

    #[test]
    fn coordinate_aft_test() {
        let length_between_perpendiculars = 125.0;
        let ship_dimensions = ShipDimensions::new(length_between_perpendiculars, 20, 0.6);
        let test_coordinate_aft = -62.5;
        assert_eq!(test_coordinate_aft, ship_dimensions.coordinate_aft());
    }

    #[test]
    fn spatium_start_coordinate_test() {
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        assert_eq!(-12.5, ship_dimensions.spatium_start_coordinate(8));
        assert_eq!(-62.5, ship_dimensions.spatium_start_coordinate(0));
        assert_eq!(56.25, ship_dimensions.spatium_start_coordinate(19));

    }
}