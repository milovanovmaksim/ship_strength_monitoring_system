pub(crate) fn round(number: f64, digits: u32) -> f64 {
    (number * 10_i32.pow(digits) as f64).round() / 10_i32.pow(digits) as f64
}


pub(crate) trait Round {
    fn my_round(&self, degets: u32) -> f64;
}

impl Round for f64 {
    fn my_round(&self, digits: u32) -> f64 {
        (self * 10_i32.pow(digits) as f64).round() / 10_i32.pow(digits) as f64
    }
}