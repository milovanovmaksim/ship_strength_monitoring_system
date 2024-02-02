pub struct Integral<'a> {
    data: &'a Vec<(f64, f64)>
}


impl<'a> Integral<'a> {
    pub fn new(data: &'a Vec<(f64, f64)>) -> Self {
        Integral { data }
    }


    pub fn integral(&self) -> f64 {
        let mut left_point = 0;
        let mut right_point = 1;
        let length = self.data.len();
        let mut integral: f64 = 0.0;
        while right_point <= length - 1 {
            let a = self.data.get(left_point).unwrap().1;
            let b = self.data.get(right_point).unwrap().1;
            let h = self.data.get(right_point).unwrap().0 - self.data.get(left_point).unwrap().0;
            integral += 0.5 * (a + b) * h.abs();
            right_point += 1;
            left_point += 1;
        }
        integral
    }

}