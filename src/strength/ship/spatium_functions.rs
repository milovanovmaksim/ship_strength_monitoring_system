use super::spatium_function::SpatiumFunction;
use serde::Deserialize;

///
/// Содержит результаты вычислений
/// (изгибающий момент, перерезывающая сила, интенсивности водоизмещения, дедвейта, сил поддержания и.т.д)
/// для всех шпаций судна.
#[derive(Debug, PartialEq, Deserialize)]
pub struct SpatiumFunctions {
    spatium_functions: Vec<SpatiumFunction>,
}

impl SpatiumFunctions {
    ///
    ///Основной конструктор.
    pub fn new(functions: Vec<SpatiumFunction>) -> Self {
        SpatiumFunctions {
            spatium_functions: functions,
        }
    }
    ///
    /// Вспомогательный конструктор.
    /// Заполняет шпации нулями.
    pub fn filled_zeros(number_spatiums: u64, lbp: f64) -> Self {
        let mut functions = vec![];
        let length_spatium = lbp / number_spatiums as f64;
        let mut start_coordinate = -lbp / 2.0;
        for id in 0..number_spatiums {
            let end_coordinate = start_coordinate + length_spatium;
            let spatium_function =
                SpatiumFunction::new(id, start_coordinate, end_coordinate, 0.0, 0.0);
            functions.push(spatium_function);
            start_coordinate += length_spatium;
        }
        SpatiumFunctions::new(functions)
    }

    ///
    /// Возвращает максимальное значение по модулю.
    pub fn max(&self) -> Option<f64> {
        if self.spatium_functions.len() == 0 {
            return None;
        }
        let mut max_value = Some(self.spatium_functions.first().unwrap().f_x1().abs());
        for s_f in &self.spatium_functions {
            let current_value = s_f.f_x1().abs().max(s_f.f_x2().abs());
            if current_value > max_value.unwrap() {
                max_value = Some(current_value);
            }
        }
        max_value
    }

    pub fn integral(&self) -> f64 {
        let mut integral = 0.0;
        for s_f in &self.spatium_functions {
            integral += s_f.integral();
        }
        integral
    }

    ///
    /// Вычисляет интеграл с переменным верхним пределом.
    pub fn integral_vul(&self) -> SpatiumFunctions {
        let mut spatium_functions = vec![];
        let mut f_x1 = 0.0;
        for s_f in &self.spatium_functions {
            let integral = s_f.integral();
            let spatium_function =
                SpatiumFunction::new(s_f.id(), s_f.x1(), s_f.x2(), f_x1, f_x1 + integral);
            f_x1 += integral;
            spatium_functions.push(spatium_function);
        }
        SpatiumFunctions { spatium_functions }
    }
    ///
    /// Возвращает значение для носа судна.
    pub fn last(&self) -> Option<&SpatiumFunction> {
        self.spatium_functions.last()
    }

    ///
    /// Добавление шпации путем сложения двух шпаций с одинаковыми id.
    /// Возвращает ссылку на измененную шпацию.
    pub fn add(&mut self, term: SpatiumFunction) {
        let id = term.id() as usize;
        if let Some(spatium_function) = self.spatium_functions.get_mut(id) {
            let new_spatium_function = spatium_function.add(term).unwrap();
            *spatium_function = new_spatium_function;
        }
    }

    ///
    /// Возвращает ссылку на шпацию по ее id.
    pub fn get(&self, id: u64) -> Option<&SpatiumFunction> {
        self.spatium_functions.get(id as usize)
    }
}

impl IntoIterator for SpatiumFunctions {
    type Item = SpatiumFunction;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.spatium_functions.into_iter()
    }
}

impl AsRef<Vec<SpatiumFunction>> for SpatiumFunctions {
    fn as_ref(&self) -> &Vec<SpatiumFunction> {
        &self.spatium_functions
    }
}
