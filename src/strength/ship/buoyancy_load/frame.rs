use serde::Deserialize;



///
/// Содержит данные масштба Бонжана для конкретного шпангоута.
/// Params:
///     id - номер шпангоута (нумерация шпангоутов c кормы в нос),
///     drafts - вектор, содержащий осадки шпангоута,
///     areas - вектор, содержащий площади погруженной части шпангоута от осадки,
///     volumes - вектор, содержащий объемы погруженной части шпангоута от осадки,
///     massa - вектор, содержащий массы погруженной части шпангоута от осадки,
///     abscissa - абсцисса шпангоута относительно центра корабля.
/// Длина всех векторов должна быть одинакова, в проивном случая будет возвращена ошибка.
#[derive(Deserialize, Debug)]
pub(crate) struct Frame {
    id: u64,
    drafts: Vec<f64>,
    areas: Vec<f64>,
    volumes: Vec<f64>,
    massa: Vec<f64>,
    abscissa: f64

}

impl Frame {
    pub fn new(id: u64, drafts: Vec<f64>, areas: Vec<f64>, volumes: Vec<f64>, massa: Vec<f64>, abscissa: f64) -> Self {
        Frame { id, drafts, areas, volumes, massa, abscissa }
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn drafts(&self) -> &Vec<f64> { &self.drafts }
}