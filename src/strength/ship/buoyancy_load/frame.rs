use log::error;
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
/// Длина всех векторов должна быть одинакова и не равна нулю, в проивном случая будет возвращена ошибка.
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
    pub fn new(id: u64, drafts: Vec<f64>, areas: Vec<f64>, volumes: Vec<f64>, masses: Vec<f64>, abscissa: f64) -> Result<Self, String> {
        match (Frame { id, drafts, areas, volumes, masses, abscissa }).validate_input_data() {
            Ok(frame) => { Ok(frame) }
            Err(err) => {
                error!("Frame::new | error: {}", err);
                Err(err)
            }
        }
    }

    fn validate_input_data(self) -> Result<Frame, String> {
        match self.empty_data_validate() {
            Ok(_) => {  },
            Err(err) => { return Err(err); }
        }
        match self.same_len_data_validate() {
            Ok(_) => { },
            Err(err) => { return Err(err); }
        }
        Ok(self)
    }

    fn empty_data_validate(&self) -> Result<(), String> {
        if self.drafts.len() == 0 {
            return Err("Вектор, содержащий осадки судна не может быть пустым.".to_string());
        }
        if self.areas.len() == 0 {
            return Err("Вектор, содержащий погруженные площади шпангоута от осадки не может быть пустым".to_string());
        }
        if self.volumes.len() == 0 {
            return Err("Вектор, содержащий погруженные объемы шпангоута от осадки не может быть пустым".to_string());
        }
        if self.masses.len() == 0 {
            return Err("Вектор, содержащий погруженные массы шпангоута от осадки не может быть пустым.".to_string());
        }
        Ok(())
    }
    fn same_len_data_validate(&self) -> Result<(), String> {
        let draft_len = self.drafts.len();
        if self.areas.len() == draft_len && self.volumes.len() == draft_len && self.masses.len() == draft_len {
            return Ok(());
        }
        Err("Длины векторов, содержащих данные масштаба Бонжана для шпангоута должны быть одинаковыми".to_string())
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn drafts(&self) -> &Vec<f64> { &self.drafts }

    pub fn abscissa(&self) ->f64 { self.abscissa }
    pub fn areas(&self) -> &Vec<f64> { &self.areas }
    pub fn volumes(&self) -> &Vec<f64> { &self.volumes }
    pub fn masses(&self) -> &Vec<f64> { &self.masses }


    ///
    /// Возвращает погруженную площадь шпангоута для заданной осадки. [м^2]
    /// Если такой осадки нет, линейно интерполирует площадь погруженного шпангоута,
    /// имея в распоряжении две известные площади шпангоутов для промежуточных осадок между
    /// которыми лежит заданная осадка.
    /// Parametrs:
    ///     draft: осадка для которой нужно вернуть погруженную площадь шпангоута.
    ///     Параметр draft  не должен выходить за пределы допустимого диапазона осадки судна.
    /// # Examples
    /// ```
    /// let id = 6;
    /// let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
    /// let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
    /// let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let abscissa =  -25.0;
    /// let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
    /// assert_eq!(0.0, frame.area_by_draft(0.5).unwrap());
    /// assert_eq!(65.12, frame.area_by_draft(2.0).unwrap());
    /// assert_eq!(32.3, frame.area_by_draft(1.0).unwrap());
    /// // Линейно интерполирует погруженную площадь шпангоута между осадками 2.0 и 3.0 метра.
    /// assert_eq!(81.605, frame.area_by_draft(2.5).unwrap());
    /// assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.area_by_draft(15.0));
    /// ```
    pub fn area_by_draft(&self, draft: f64) -> Result<f64, String> {
        match self.data_by_draft(draft, &self.areas) {
            Ok(area) => { Ok(area) },
            Err(error) => {
                error!("Frame::area_by_draft | error: {}", error);
                Err(error)
            }
        }
    }

    ///
    /// Возвращает данные масштаба Бонжана для заданной осадки.
    fn data_by_draft(&self, draft: f64, data: &Vec<f64>) -> Result<f64, String> {
        let min_draft = *self.drafts.first().unwrap();
        if draft < min_draft {
            return Ok(0.0);
        }
        match self.validate_draft(draft) {
            Ok(_) => {
                match self.drafts.custom_binary_search(draft) {
                    (Some(left_point), Some(right_point)) => {
                        let draft_0 = *self.drafts.get(left_point).unwrap();
                        let draft_1 = *self.drafts.get(right_point).unwrap();
                        let f_x_0 = *data.get(left_point).unwrap();
                        let f_x_1 = *data.get(right_point).unwrap();
                        let linear_interpolated = LinearInterpolation::new(f_x_0, f_x_1, draft_0,draft_1);
                        match linear_interpolated.interpolated_value(draft) {
                            Ok(value) => { Ok(value) },
                            Err(error) => {
                                error!("Frame::data_by_draft | error: {}", error);
                                Err(error)
                            }
                        }
                    },
                    (Some(middle), None) => { Ok(*data.get(middle).unwrap()) },
                    _ => { unreachable!("Осадка находится в допустимом диапазоне.
                        Пустые векторы, содержащие данные масштаба Бонжана для шпангоута не допускаются.") }
                }
            },
            Err(error) => {
                error!("Frame::data_by_draft | {}", error);
                Err(error)
            }
        }
    }

    ///
    /// Валидация осадки.
    fn validate_draft(&self, draft: f64) -> Result<(), String> {
        let max_draft = *self.drafts.last().unwrap();
        if draft > max_draft {
            return Err(format!("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: {} [м].", max_draft));
        }
        Ok(())
    }


    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки. [м^3]
    /// Если такой осадки нет, линейно интерполирует объем погруженного шпангоута,
    /// имея в распоряжении два известных объема шпангоутов для промежуточных осадок между
    /// которыми лежит заданная осадка.
    /// Parametrs:
    ///     draft: осадка для которой нужно вернуть погруженный объем шпангоута.
    ///     Параметр draft  не должен выходить за пределы допустимого диапазона осадки судна.
    /// # Examples
    /// ```
    /// let id = 6;
    /// let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
    /// let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
    /// let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let abscissa =  -25.0;
    /// let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
    /// assert_eq!(0.0, frame.volume_by_draft(0.5).unwrap());
    /// assert_eq!(379.52, frame.volume_by_draft(1.0).unwrap());
    /// assert_eq!(765.20, frame.volume_by_draft(2.0).unwrap());
    /// assert_eq!(5146.22, frame.volume_by_draft(13.3).unwrap());
    /// assert_eq!(5029.90, frame.volume_by_draft(13.0).unwrap());
    /// // Линейно интерполирует погруженный объем шпангоута между осадками 2.0 и 3.0 метра.
    /// assert_eq!(958.855, frame.volume_by_draft(2.5).unwrap());
    /// assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.volume_by_draft(15.0));
    /// ```
    pub fn volume_by_draft(&self, draft: f64) -> Result<f64, String> {
        match self.data_by_draft(draft, &self.volumes) {
            Ok(volume) => { Ok(volume) },
            Err(error) => {
                error!("Frame::volume_by_draft | error: {}", error);
                Err(error)
            }
        }
    }

    ///
    /// Возвращает погруженную массу шпангоута для заданной осадки. [т]
    /// Если такой осадки нет, линейно интерполирует массу погруженного шпангоута,
    /// имея в распоряжении две известные массы шпангоутов для промежуточных осадок между
    /// которыми лежит заданная осадка.
    /// Parametrs:
    ///     draft: осадка для которой нужно вернуть погруженную массу шпангоута.
    ///     Параметр draft  не должен выходить за пределы допустимого диапазона осадки судна.
    /// # Examples
    /// ```
    /// let id = 6;
    /// let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
    /// let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
    /// let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let abscissa =  -25.0;
    /// let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
    /// assert_eq!(0.0, frame.massa_by_draft(0.5).unwrap());
    /// assert_eq!(379.52, frame.massa_by_draft(1.0).unwrap());
    /// assert_eq!(765.20, frame.massa_by_draft(2.0).unwrap());
    /// assert_eq!(5146.22, frame.massa_by_draft(13.3).unwrap());
    /// assert_eq!(5029.90, frame.massa_by_draft(13.0).unwrap());
    /// // Линейно интерполирует погруженную массу шпангоута между осадками 2.0 и 3.0 метра.
    /// assert_eq!(958.855, frame.massa_by_draft(2.5).unwrap());
    /// assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.massa_by_draft(15.0));
    /// ```
    pub fn massa_by_draft(&self, draft: f64) -> Result<f64, String> {
        match self.data_by_draft(draft, &self.masses) {
            Ok(massa) => { Ok(massa) },
            Err(error) => {
                error!("Frame::massa_by_draft | error: {}", error);
                Err(error)
            }
        }
    }
}
