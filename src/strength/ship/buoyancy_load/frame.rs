

///
/// Масштб Бонжана для конкретного шпангоута.
/// Params:
///     id - номер шпангоута (нумерация шпангоутов слева направо),
///     drafts - вектор, содержащий осадки шпангоута,
///     areas - вектор, содержащий площади погруженной части шпангоута от осадки,
///     volumes - вектор, содержащий объемы погруженной части шпангоута от осадки,
///     massa - вектор, содержащий массы погруженной части шпангоута от осадки,
///     x, y, z - вектора, содержащие координаты центра тяжести погруженной части шпангоута.
/// Длина всех векторов должна быть одинакова, в проивном случая будет возвращена ошибка.
#[derive(Debug)]
struct Frame {
    id: u64,
    drafts: Vec<f64>,
    areas: Vec<f64>,
    volumes: Vec<f64>,
    massa: Vec<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>
}

impl Frame {
    pub fn new(id: u64, drafts: Vec<f64>, areas: Vec<f64>, volumes: Vec<f64>, massa: Vec<f64>,
               x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) -> Self {
        Frame { id, drafts, areas, volumes, massa, x, y, z }
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn drafts(&self) -> &Vec<f64> { &self.drafts }
}