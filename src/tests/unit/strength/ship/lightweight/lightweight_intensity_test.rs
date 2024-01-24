#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use log::debug;

    use crate::strength::ship::{lightweight::lightweight::LightweightIntensity, ship_dimensions::ShipDimensions};



    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                let _ = env_logger::try_init();
            }
        )
    }

    #[test]
    fn create_lightweight_from_json_file_successfully() {
        let lightweight = LightweightIntensity::from_json_file("./src/tests/unit/strength/lightweight/data/correct_data.json".to_string());
        assert!(lightweight.is_ok());
    }

    #[test]
    fn create_lightweight_from_json_file_invalid_type() {
        let lightweight = LightweightIntensity::from_json_file("./src/tests/unit/strength/lightweight/data/invalid_type.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("invalid type"));
    }

    #[test]
    fn create_lightweight_from_json_file_missing_field() {
        let lightweight = LightweightIntensity::from_json_file("./src/tests/unit/strength/lightweight/data/empty_field.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("missing field `lightweight`"));
    }
    #[test]
    fn test_number_spatiums() {
        let number_spatiums = 20;
        let ship_dimensions = ShipDimensions::new(125.03, number_spatiums, 0.8);
        let lightweight = LightweightIntensity::new(1750.0, ship_dimensions);
        let output = lightweight.lightweight_intensity();
        assert_eq!(output.len(), number_spatiums as usize);
    }

    #[test]
    fn test_lightweight_intensity() {
        call_once();
        let test_weight = 13575.73;
        let ship_dimensions = ShipDimensions::new(235.03, 20, 0.5);
        let test_lightweight = LightweightIntensity::new(test_weight, ship_dimensions);
        let output = test_lightweight.lightweight_intensity();
        let mut computed_weight = 0.0;
        for spatium in &output {
            computed_weight += spatium.integral();
        }
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