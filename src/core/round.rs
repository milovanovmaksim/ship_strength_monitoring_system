pub(crate) trait Round {
    fn my_round(&self, degets: i32) -> f64;
}

impl Round for f64 {
    fn my_round(&self, digits: i32) -> f64 {
        let scale = 10_f64.powi(digits);
        (self * scale).round() / scale
    }
}
