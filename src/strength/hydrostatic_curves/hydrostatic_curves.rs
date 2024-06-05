use log::{debug, error};
use serde::Deserialize;

use crate::core::{
    binary_search::BinarySearch, json_file::JsonFile, linear_interpolation::LinearInterpolation,
};

use super::hydrostatic_typedata::HydrostaticTypeData;

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
///     x_f: абсцисса центра тяжести ватерлиниии,
///     r_l - продольный(большой) метацентрический радиус.
#[derive(Deserialize, Debug)]
pub(crate) struct HydrostaticCurves {
    drafts: Vec<f64>,
    displacement_tonnage: Vec<f64>,
    x_c: Vec<f64>,
    waterline_area: Vec<f64>,
    x_f: Vec<f64>,
    r_l: Vec<f64>,
}

impl HydrostaticCurves {
    ///
    /// Основной конструктор.
    pub fn new(
        drafts: Vec<f64>,
        displacement_tonnage: Vec<f64>,
        x_c: Vec<f64>,
        waterline_area: Vec<f64>,
        x_f: Vec<f64>,
        r_l: Vec<f64>,
    ) -> Result<Self, String> {
        match (HydrostaticCurves {
            drafts,
            displacement_tonnage,
            x_c,
            waterline_area,
            x_f,
            r_l,
        })
        .validate_data()
        {
            Ok(hydrostatic_curves) => Ok(hydrostatic_curves),
            Err(err) => {
                error!("HydrostaticCurves::new | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<HydrostaticCurves, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => match serde_json::from_reader(content) {
                Ok(value) => {
                    debug!("HydrostaticCurves::from_json_file | HydrostaticCurves has been created sucessfuly.");
                    Ok(value)
                }
                Err(err) => {
                    error!("HydrostaticCurves::from_json_file | error: {:?}.", err);
                    return Err(err.to_string());
                }
            },
            Err(err) => {
                error!("HydrostaticCurves::from_json_file | error: {:?}.", err);
                return Err(err);
            }
        }
    }

    ///
    /// Валидация входных данных.
    fn validate_data(self) -> Result<HydrostaticCurves, String> {
        if let Err(err) = self.validate_empty_data() {
            return Err(err);
        }
        if let Err(err) = self.validate_same_length() {
            return Err(err);
        }
        if let Err(err) = self.validate_more_zero() {
            return Err(err);
        }
        if let Err(err) = self.validate_drafts() {
            return Err(err);
        }
        Ok(self)
    }


    ///
    /// Валидация: все элементы теоретического чертежа должны быть заданы.
    fn validate_empty_data(&self) -> Result<(), String> {
        if self.drafts.len() == 0
            || self.displacement_tonnage.len() == 0
            || self.x_c.len() == 0
            || self.waterline_area.len() == 0
            || self.x_f.len() == 0
            || self.r_l.len() == 0
        {
            return Err("Гидростатические кривые не заданы".to_string());
        }
        Ok(())
    }

    fn validate_drafts(&self) -> Result<(), String>{
        let priviuse_draft = self.drafts.first().unwrap();
        for draft in self.drafts[1..].iter() {
            if draft < priviuse_draft {
                return Err("Осадка `drafts` должна быть отсортированна по возрастанию и не содержать повторяющихся значений.".to_string());
            }
        }
        Ok(())
    }

    ///
    /// Валидация: массивы, содержащие данные элементов теоретического чертежа должны иметь одинаковую длину.
    fn validate_same_length(&self) -> Result<(), String> {
        let drafts_len = self.drafts.len();
        if drafts_len == self.displacement_tonnage.len()
            && drafts_len == self.x_c.len()
            && drafts_len == self.waterline_area.len()
            && drafts_len == self.x_f.len()
            && drafts_len == self.r_l.len()
        {
            return Ok(());
        }
        Err("Массивы значений элементов теоретического чертежа имеют разную длину.".to_string())
    }

    ///
    /// Валидация: осадка судна (drafts), площадь ватерлинии (waterline_area),
    /// весовое водоизмещение (displacement_tonnage) должны быть больше нуля.
    fn validate_more_zero(&self) -> Result<(), String> {
        let more_than_zero = |data: &Vec<f64>| -> bool {
            for item in data {
                if *item < 0.0 {
                    return false;
                }
            }
            true
        };

        if more_than_zero(&self.drafts)
            && more_than_zero(&self.displacement_tonnage)
            && more_than_zero(&self.waterline_area)
            && more_than_zero(&self.r_l)
        {
            return Ok(());
        }
        Err("Осадка судна (drafts), площадь ватерлинии (waterline_area), весовое водоизмещение (displacement_tonnage) должны быть больше нуля.".to_string())
    }

    ///
    /// Валидация: осадка не должна превышать осадку судна в полном грузу.
    /// Parameters:
    ///     draft - осадка судна.
    fn validate_draft(&self, draft: f64) -> Result<(), String> {
        let max_draft = *self.drafts.last().unwrap();
        if draft > max_draft {
            return Err(format!("Осадка превысила максимальную осадку для данного судна. Максимальная осадка по гидростатическим кривым составляет: {}, передано значение: {}", max_draft, draft));
        }
        Ok(())
    }

    ///
    /// Валидация: весовое водоизмещение не должно превышать максимальное весовое водоизмещение судна в полном грузу.
    /// Parameters:
    ///     dispalcement_tonnage - весовое вододоизмещение.
    fn validate_dispalcement_tonnage(&self, dispalcement_tonnage: f64) -> Result<(), String> {
        let max_dispalcement_tonnage = *self.displacement_tonnage.last().unwrap();
        if dispalcement_tonnage > max_dispalcement_tonnage {
            return Err(format!("Весовое водоизмещение превысило максимальное водоизмещение судна в полном грузу. Максимальное весовое водоизмещение судна в полном грузу составляет: {}, передано значение: {}", max_dispalcement_tonnage, dispalcement_tonnage));
        }
        Ok(())
    }

    ///
    /// Возвращает осадку судна по заданному весовому водоизмещению [м].
    /// Если весовое водоизмещение меньше чем весовое водоизмещение судна порожнем, возвращает 0.0.
    /// Parameters:
    ///     dispalcement_tonnage - весовое вододоизмещение.
    pub fn draft_by_displacement_tonnage(&self, dispalcement_tonnage: f64) -> Result<f64, String> {
        match self.validate_dispalcement_tonnage(dispalcement_tonnage) {
            Ok(_) => {
                match self
                    .displacement_tonnage
                    .custom_binary_search(dispalcement_tonnage)
                {
                    (Some(left_id), Some(right_id)) => {
                        let linear_interpolation = LinearInterpolation::new(
                            *self.drafts.get(left_id).unwrap(),
                            *self.drafts.get(right_id).unwrap(),
                            *self.displacement_tonnage.get(left_id).unwrap(),
                            *self.displacement_tonnage.get(right_id).unwrap(),
                        );
                        match linear_interpolation.interpolated_value(dispalcement_tonnage) {
                            Ok(draft) => Ok(draft),
                            Err(error) => {
                                error!(
                                    "HydrostaticCurves::draft_by_displacement_tonnage | {}",
                                    error
                                );
                                Err(error)
                            }
                        }
                    }
                    (Some(id), None) => Ok(*self.drafts.get(id).unwrap()),
                    _ => Ok(0.0),
                }
            }
            Err(error) => {
                error!(
                    "HydrostaticCurves::draft_by_displacement_tonnage | {}",
                    error
                );
                Err(error)
            }
        }
    }


    ///
    /// Возвращает данные элементов теоретического чертежа от осадки судна.
    /// Parameters:
    ///     draft - осадка судна,
    ///     type_data - enum HydrostaticTypeData
    pub fn get_data_by_draft(
        &self,
        draft: f64,
        type_data: HydrostaticTypeData,
    ) -> Result<f64, String> {
        let data = {
            match type_data {
                HydrostaticTypeData::LCB => &self.x_c,
                HydrostaticTypeData::LCF => &self.x_f,
                HydrostaticTypeData::WaterlineArea => &self.waterline_area,
                HydrostaticTypeData::LongitudinalMetacentricRadius => &self.r_l,
            }
        };
        match self.validate_draft(draft) {
            Ok(_) => match self.drafts.custom_binary_search(draft) {
                (Some(left_index), Some(right_index)) => {
                    let linear_interpolation = LinearInterpolation::new(
                        *data.get(left_index).unwrap(),
                        *data.get(right_index).unwrap(),
                        *self.drafts.get(left_index).unwrap(),
                        *self.drafts.get(right_index).unwrap(),
                    );
                    match linear_interpolation.interpolated_value(draft) {
                        Ok(value) => {
                            return Ok(value);
                        }
                        Err(error) => {
                            error!("HydrostaticCurves::get_data_by_draft | {}", error);
                            return Err(error);
                        }
                    }
                }
                (Some(index), None) => Ok(*data.get(index).unwrap()),
                _ => { Ok(0.0) }
            },
            Err(error) => {
                error!("HydrostaticCurves::get_data_by_draft | {}", error);
                Err(error)
            }
        }
    }
}
