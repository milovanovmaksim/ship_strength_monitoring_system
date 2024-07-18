#[cfg(test)]
mod tests {
    use crate::core::water_density::WaterDensity;
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
    fn div_test() {
        call_once();
        let water_density = WaterDensity::new(4.0);
        assert_eq!(2.0, 8.0 / water_density);
    }
}
