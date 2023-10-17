#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use log::debug;
    use crate::strength::{ship::{loads::lightweight::lightweight::Lightweight, ship_dimensions::ShipDimensions}, output::type_output::TypeOutput};


    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                env_logger::init();
            }
        )
    }


    #[test]
    fn create_lightweight_from_json_file_successfully() {
        call_once();
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/correct_data.json".to_string());
        assert!(lightweight.is_ok());
    }

    #[test]
    fn create_lightweight_from_json_file_invalid_type() {
        call_once();
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/invalid_type.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("invalid type"));
    }

    #[test]
    fn create_lightweight_from_json_file_missing_field() {
        call_once();
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/empty_field.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("missing field `lightweight`"));
    }
    #[test]
    fn test_number_spatiums() {
        call_once();
        let number_spatiums = 20;
        let ship_dimensions = ShipDimensions::new(125.03, number_spatiums, 0.8);
        let lightweight = Lightweight::new(1750.0, ship_dimensions);
        let output = lightweight.lightweight_intensity();
        assert_eq!(output.len(), number_spatiums as usize);
    }

    #[test]
    fn test_type_output() {
        call_once();
        let ship_dimensions = ShipDimensions::new(125.0,20, 0.7);
        let lightweight = Lightweight::new(1750.0, ship_dimensions);
        let output = lightweight.lightweight_intensity();
         assert_eq!(output.type_output(), TypeOutput::LightweightIntensity);
    }

    #[test]
    fn test_lightweight_intensity() {
        call_once();
        let test_weight = 13575.73;
        let ship_dimensions = ShipDimensions::new(235.03, 20, 0.65);
        let test_lightweight = Lightweight::new(test_weight, ship_dimensions);
        let output = test_lightweight.lightweight_intensity();
        let computed_weight = output.integral();
        let err = {
            if computed_weight > test_weight {
                ((computed_weight - test_weight) / test_weight) * 100.0

            } else if test_weight > computed_weight {
                ((test_weight - computed_weight) / computed_weight) * 100.0
            } else {
                0.0
            }
        };
        debug!("\nОтносительная ошибка численного интегрирования интенсивности веса корпуса корабля = {} %", err);
        // Lightweight расчитанное, не должно отличаться от заданного более чем на 0.5%.
        assert!(err < 0.5, "Error more than 0.5% = {}%.", err);
    }
}