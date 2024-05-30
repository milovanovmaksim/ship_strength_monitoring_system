#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use log::debug;

    use crate::strength::ship::{lightweight::lightweight_intensity::LightweightIntensity, ship_dimensions::ShipDimensions};



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
    fn test_lightweight_intensity() {
        call_once();
        let test_weight = 1357.73;
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.5);
        let test_lightweight = LightweightIntensity::new(&ship_dimensions);
        let lightweight_intensity = test_lightweight.spatium_functions(test_weight);
        let mut computed_weight = 0.0;
        for spatium in lightweight_intensity.as_ref() {
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