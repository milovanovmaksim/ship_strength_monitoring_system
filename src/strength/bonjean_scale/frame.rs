use crate::core::{binary_search::BinarySearch, linear_interpolation::LinearInterpolation};
use serde::Deserialize;
use tracing::instrument;

///
/// Содержит данные масштба Бонжана для конкретного шпангоута.
/// Params:
///     id - номер шпангоута (нумерация шпангоутов c кормы в нос),
///     drafts - вектор, содержащий осадки шпангоута, вектор должен быть отсортирован по возрастанию,
///     areas - вектор, содержащий площади погруженной части шпангоута от осадки,
///     volumes - вектор, содержащий объемы погруженной части шпангоута от осадки,
///     masses - вектор, содержащий массы погруженной части шпангоута от осадки,
///     abscissa - абсцисса шпангоута относительно центра корабля.
/// Длина всех векторов должна быть одинакова и не равна нулю, в проивном случая будет возвращена ошибка.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Frame {
    id: u64,
    drafts: Vec<f64>,
    areas: Vec<f64>,
    abscissa: f64,
}

impl Frame {
    ///
    /// Основной конструктор.
    #[instrument(err, target = "Frame::new")]
    pub fn new(id: u64, drafts: Vec<f64>, areas: Vec<f64>, abscissa: f64) -> Result<Self, String> {
        (Frame {
            id,
            drafts,
            areas,
            abscissa,
        })
        .validate_input_data()
    }

    ///
    /// Абсцисса шпангоута относительно центра корабля.
    pub fn abscissa(&self) -> f64 {
        self.abscissa
    }

    //
    // Валидация входных данных.
    fn validate_input_data(self) -> Result<Frame, String> {
        if let Err(err) = self.empty_data_validate() {
            return Err(err);
        }
        if let Err(err) = self.same_length_data_validate() {
            return Err(err);
        }
        Ok(self)
    }

    //
    // Валидция входных данных.
    // Векторы, содержащие данные масштаба Бонжана для шпангоута, не должны быть пустыми.
    fn empty_data_validate(&self) -> Result<(), String> {
        if self.drafts.len() == 0 {
            return Err("Вектор, содержащий осадки судна, не может быть пустым.".to_string());
        }
        if self.areas.len() == 0 {
            return Err(
                "Вектор, содержащий погруженные площади шпангоута от осадки, не может быть пустым"
                    .to_string(),
            );
        }
        Ok(())
    }

    //
    // Валидация входных данных.
    // Векторы, содержащие данные масштаба Бонжана для шпангоута, должны иметь одинаковую длину.
    fn same_length_data_validate(&self) -> Result<(), String> {
        let draft_len = self.drafts.len();
        if self.areas.len() == draft_len {
            return Ok(());
        }
        Err("Длины векторов, содержащих данные масштаба Бонжана для шпангоута, должны быть одинаковыми".to_string())
    }

    ///
    /// Возвращает площадь погруженной части шпангоута от осадки.
    /// Если такой осадки нет, линейно интерполирует площадь шпангоута
    /// для промежуточных осадок, между которыми лежит заданная осадка.
    /// # Example
    /// ```
    /// let id = 6;
    /// let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
    /// let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
    /// let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
    /// let abscissa =  -25.0;
    /// let frame = Frame::new(id, drafts.clone(), areas.clone(), volumes, masses, abscissa).unwrap();
    /// assert_eq!(0.0, frame.data_by_draft(0.5, BonjeanScaleDataType::Area).unwrap());
    /// for i in 0..drafts.len() {
    ///    let draft = *drafts.get(i).unwrap();
    ///    assert_eq!(*areas.get(i).unwrap(), frame.data_by_draft(draft, BonjeanScaleDataType::Area).unwrap());
    /// }
    /// //Линейно интерполирует погруженную площадь шпангоута между осадками 2.0 и 3.0 метра.
    /// assert_eq!(81.605, frame.data_by_draft(2.5, BonjeanScaleDataType::Area).unwrap());
    /// ```
    #[instrument(skip(self), err, target = "Frame::area_by_draft")]
    pub fn area_by_draft(&self, draft: f64) -> Result<f64, String> {
        if draft < self.min_draft() {
            return Ok(0.0);
        } else if draft > self.max_draft() {
            return Err(format!("Осадка превысила осадку судна в грузу."));
        }
        match self.drafts.custom_binary_search(draft) {
            (Some(left_point), Some(right_point)) => {
                let linear_interpolated = LinearInterpolation::new(
                    *self.areas.get(left_point).unwrap(),
                    *self.areas.get(right_point).unwrap(),
                    *self.drafts.get(left_point).unwrap(),
                    *self.drafts.get(right_point).unwrap(),
                );
                linear_interpolated.interpolated_value(draft)
            }
            (Some(middle), None) => Ok(*self.areas.get(middle).unwrap()),
            _ => {
                unreachable!("Осадка находится в заданном диапазоне.
                    Пустые векторы, содержащие данные масштаба Бонжана для шпангоута, не допускаются.")
            }
        }
    }

    /// Максимальная осадка для данного шпангоута.
    fn max_draft(&self) -> f64 {
        *self.drafts.last().unwrap()
    }

    fn min_draft(&self) -> f64 {
        *self.drafts.first().unwrap()
    }
}
