
///
/// Линейная интерполяция.
/// Вычисляет значение функции, имея в распоряжении f(x) в двух различных точках f_x_0, f_x_1.
/// Parameters:
///     f_x_0, f_x_1 - значения функции f(x) в точке x_0 и точке x_1;
///     x_0, x_1 - точки в которых определена функция f(x).
pub(crate) struct LinearInterpolation {
    f_x_0: f64,
    f_x_1: f64,
    x_0: f64,
    x_1: f64
}

impl LinearInterpolation {
    pub fn new(f_x_0: f64, f_x_1: f64, x_0: f64, x_1: f64) -> Self {
        LinearInterpolation {f_x_0, f_x_1, x_0, x_1}
    }

    ///
    /// Вычисляет значение функции в точке x, имея в распоряжении f(x) в двух различных точках f_x_0, f_x_1.
    /// Примечание:
    ///    x_0 < x < x_1
    pub fn interpolated_value(&self, x: f64) -> Result<f64, String> {
        if self.x_0 < x && x < self.x_1 {
            let f_x = self.f_x_0 + ((self.f_x_1 - self.f_x_0) / (self.x_1-self.x_0)) * (x - self.x_0);
            return Ok(f_x);
        }
        Err("Function argument 'x' should be x_0 < x < x_1.".to_owned())

    }
}