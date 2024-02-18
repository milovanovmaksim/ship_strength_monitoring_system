use serde::Deserialize;

use crate::core::{binary_search::BinarySearch, linear_interpolation::LinearInterpolation};



///
/// Содержит данные масштба Бонжана для конкретного шпангоута.
/// Params:
///     id - номер шпангоута (нумерация шпангоутов c кормы в нос),
///     drafts - вектор, содержащий осадки шпангоута,
///     areas - вектор, содержащий площади погруженной части шпангоута от осадки,
///     volumes - вектор, содержащий объемы погруженной части шпангоута от осадки,
///     masses - вектор, содержащий массы погруженной части шпангоута от осадки,
///     abscissa - абсцисса шпангоута относительно центра корабля.
/// Длина всех векторов должна быть одинакова, в проивном случая будет возвращена ошибка.
#[derive(Deserialize, Debug)]
pub(crate) struct Frame {
    id: u64,
    drafts: Vec<f64>,
    areas: Vec<f64>,
    volumes: Vec<f64>,
    masses: Vec<f64>,
    abscissa: f64

}

impl Frame {
    pub fn new(id: u64, drafts: Vec<f64>, areas: Vec<f64>, volumes: Vec<f64>, masses: Vec<f64>, abscissa: f64) -> Self {
        Frame { id, drafts, areas, volumes, masses, abscissa }
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn drafts(&self) -> &Vec<f64> { &self.drafts }

    pub fn abscissa(&self) ->f64 { self.abscissa }
    pub fn areas(&self) -> &Vec<f64> { &self.areas }
    pub fn volumes(&self) -> &Vec<f64> { &self.volumes }
    pub fn masses(&self) -> &Vec<f64> { &self.masses }


    ///
    /// Возвращает погруженную площадь шпангоута по осадке [м^2].
    /// Если такой осадки нет, линейно интерполирует площадь погруженного шпангоута,
    /// имея в распоряжении две ближайшие известные осадки судна.
    /// Parametrs:
    ///     draft: осадка для которой нужно вернуть погруженную площадь шпангоута.
    ///     Параметр draft  не должен выходить за пределы допустимого диапазона осадки судна.
    pub fn area_by_draft(&self, draft: f64) -> Result<f64, String> {
        let min_draft = *self.drafts.first().unwrap();
        let max_draft = *self.drafts.last().unwrap();
        if draft < 0.0 {
            return Err("Осадка судна не может быть отрицательной.".to_string());
        } else if draft > max_draft {
            return Err(format!("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: {} метра.", max_draft));
        }
        if draft < min_draft {
            return Err(format!("Осадка меньше чем минимально известная для данного шпангоута. Минимальная осадка: {} метра.", min_draft));
        }

        match self.drafts.custom_binary_search(draft) {
            (Some(left_point), Some(right_point)) => {
                let draft_0 = *self.drafts.get(left_point).unwrap();
                let draft_1 = *self.drafts.get(right_point).unwrap();
                let area_0 = *self.areas.get(left_point).unwrap();
                let area_1 = *self.areas.get(right_point).unwrap();
                let linear_interpolated = LinearInterpolation::new(area_0, area_1, draft_0,draft_1);
                return linear_interpolated.interpolated_value(draft);
            },
            (Some(middle), None) => { return Ok(*self.areas.get(middle).unwrap()); }
            _ => { unreachable!("Осадка находится в допустимом диапазоне") }
        }
    }
}