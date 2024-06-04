use log::error;

use crate::core::{binary_search::BinarySearch, linear_interpolation::LinearInterpolation};


///
/// HydrostaticCurves(гидростатические кривые) - кривые элементов теоретического чертежа, графические зависимости
/// от осадки судна характеристик его плавучести и начальной остойчивости: площади ватерлинии, ее моментов инерции
/// и центра тяжести, водоизмещения, положения центра величины по длине и по высоте, возвышения поперечного
/// и продольного метацентров над килем.
/// Paramenters:
///     drafts: осадка, вектор должен быть отсортирован по возрастанию,
///     displacement: Весовое водоизмещение,
///     x_c: абсцисса центра велечины,
///     waterline_area: площадь ватерлинии,
///     x_f: абсцисса центра тяжести ватерлиниии
pub(crate) struct HydrostaticCurves {
    drafts: Vec<f64>,
    displacement_tonnage: Vec<f64>,
    x_c: Vec<f64>,
    waterline_area: Vec<f64>,
    x_f: Vec<f64>,
}


impl HydrostaticCurves {
    pub fn new(mut drafts: Vec<f64>, displacement_tonnage: Vec<f64>, x_c: Vec<f64>, waterline_area: Vec<f64>, x_f: Vec<f64>) -> Result<Self, String> {
        match (HydrostaticCurves { drafts, displacement_tonnage, x_c, waterline_area, x_f }).validate_data() {
            Ok(hydrostatic_curves) => { Ok(hydrostatic_curves) }
            Err(err) => {
                error!("HydrostaticCurves::new | error: {}", err);
                Err(err)
            }
        }
    }

    fn validate_data(mut self) -> Result<HydrostaticCurves, String> {
        self.drafts.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if let Err(err) = self.validate_empty_data() {
            return Err(err);
        }
        if let Err(err) = self.validate_same_length() {
            return Err(err);
        }
        if let Err(err) = self.validate_more_zero() {
            return Err(err);
        }
        Ok(self)
    }

    fn validate_empty_data(&self) -> Result<(), String> {
        if self.drafts.len() == 0 || self.displacement_tonnage.len() == 0 || self.x_c.len() == 0 || self.waterline_area.len() == 0 || self.x_f.len() == 0 {
           return Err("Гидростатические кривые не заданы".to_string());
        }
        Ok(())
    }

    fn validate_same_length(&self) -> Result<(), String> {
        let drafts_len = self.drafts.len();
        if drafts_len == self.displacement_tonnage.len() && drafts_len == self.x_c.len() && drafts_len == self.waterline_area.len() && drafts_len == self.x_f.len() {
            return Ok(());
        }
        Err("Массивы значений элементов теоретического чертежа имеют разную длину.".to_string())
    }

    fn validate_more_zero(&self) -> Result<(), String> {
        let more_zero = |data: &Vec<f64>| -> bool {
            for item in data {
                if *item < 0.0 {
                    return false;
                }
            }
            true
        };

        if more_zero(&self.drafts) && more_zero(&self.displacement_tonnage)
            && more_zero(&self.waterline_area) {
            return Ok(());
        }
        Err("Осадка судна (drafts), площадь ватерлинии (waterline_area), весовое водоизмещение (displacement_tonnage) должны быть больше нуля.".to_string())
    }

    fn validate_draft(&self, draft: f64) -> Result<(), String> {
        let max_draft = *self.drafts.last().unwrap();
        if draft > max_draft {
            return Err(format!("Осадка превысила максимальную осадку для данного судна. Максимальная осадка по гидростатическим кривым составляет: {}, передано значение: {}", max_draft, draft));
        }
        Ok(())
    }

    fn validate_dispalcement_tonnage(&self, dispalcement_tonnage: f64) -> Result<(), String> {
        let max_dispalcement_tonnage = *self.displacement_tonnage.last().unwrap();
        if dispalcement_tonnage > max_dispalcement_tonnage {
            return Err(format!("Весовое водоизмещение превысило максимальное водоизмещение для данного судна. Максимальное весовое водоизмещение по гидростатическим кривым составляет: {}, передано значение: {}", max_dispalcement_tonnage, dispalcement_tonnage));
        }
        Ok(())
    }

    ///
    /// Возвращает осадку судна по заданному весовому водоизмещению [м].
    pub fn draft_by_displacement_tonnage(&self, dispalcement_tonnage: f64) -> Result<f64, String> {
        match self.validate_dispalcement_tonnage(dispalcement_tonnage) {
            Ok(_) => {
                match self.displacement_tonnage.custom_binary_search(dispalcement_tonnage) {
                    (Some(left_id), Some(right_id)) => {
                        let linear_interpolation = LinearInterpolation::new(
                            *self.drafts.get(left_id).unwrap(),
                            *self.drafts.get(right_id).unwrap(),
                            *self.displacement_tonnage.get(left_id).unwrap(),
                            *self.displacement_tonnage.get(right_id).unwrap()
                        );
                        match linear_interpolation.interpolated_value(dispalcement_tonnage) {
                            Ok(draft) => { Ok(draft) }
                            Err(error) => {
                                error!("HydrostaticCurves::draft_by_displacement_tonnage | {}", error);
                                Err(error)
                            }
                        }
                    }
                    (Some(id), None) => { Ok(*self.drafts.get(id).unwrap()) }
                    _ => unreachable!("Весовое водоизмещение находится в заданном диапазоне.")
                }
            }
            Err(error) => {
                error!("HydrostaticCurves::draft_by_displacement_tonnage | {}", error);
                Err(error)
            }
        }
    }




}