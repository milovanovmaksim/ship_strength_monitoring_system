use log::{debug, error};
use serde::Deserialize;
use tracing::instrument;

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
///     displacement: весовое водоизмещение,
///     x_c: абсцисса центра велечины,
///     waterline_area: площадь ватерлинии,
///     x_f: абсцисса центра тяжести ватерлиниии,
///     lmr - продольный(большой) метацентрический радиус.
#[derive(Deserialize, Debug)]
pub struct HydrostaticCurves {
    drafts: Vec<f64>,
    displacement_tonnage: Vec<f64>,
    x_c: Vec<f64>,
    waterline_area: Vec<f64>,
    x_f: Vec<f64>,
    lmr: Vec<f64>,
}

impl HydrostaticCurves {
    ///
    /// Основной конструктор.
    #[instrument(skip_all, target = "HydrostaticCurves::new")]
    pub fn new(
        drafts: Vec<f64>,
        displacement_tonnage: Vec<f64>,
        x_c: Vec<f64>,
        waterline_area: Vec<f64>,
        x_f: Vec<f64>,
        r_l: Vec<f64>,
    ) -> Result<Self, String> {
        (HydrostaticCurves {
            drafts,
            displacement_tonnage,
            x_c,
            waterline_area,
            x_f,
            lmr: r_l,
        })
        .validate_input_data()
    }

    ///
    /// Вспомогательный конструктор.
    #[instrument(skip_all, target = "HydrostaticCurves::from_json_file")]
    pub fn from_json_file(file_path: String) -> Result<HydrostaticCurves, String> {
        let json = JsonFile::new(file_path);
        let content = json.content()?;
        serde_json::from_reader(content).map_err(|err| err.to_string())
    }

    ///
    /// Валидация входных данных.
    fn validate_input_data(self) -> Result<HydrostaticCurves, String> {
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
            || self.lmr.len() == 0
        {
            return Err("Гидростатические кривые не заданы".to_string());
        }
        Ok(())
    }

    fn validate_drafts(&self) -> Result<(), String> {
        let priviuse_draft = self.drafts.first().unwrap();
        for draft in self.drafts[1..].iter() {
            if draft < priviuse_draft {
                return Err("Осадка `drafts` должна быть отсортирована по возрастанию и не содержать повторяющихся значений.".to_string());
            }
        }
        Ok(())
    }

    ///
    /// Валидация: массивы, содержащие данные элементов теоретического чертежа, должны иметь одинаковую длину.
    fn validate_same_length(&self) -> Result<(), String> {
        let drafts_len = self.drafts.len();
        if drafts_len == self.displacement_tonnage.len()
            && drafts_len == self.x_c.len()
            && drafts_len == self.waterline_area.len()
            && drafts_len == self.x_f.len()
            && drafts_len == self.lmr.len()
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
            && more_than_zero(&self.lmr)
        {
            return Ok(());
        }
        Err("Осадка судна (drafts), площадь ватерлинии (waterline_area), весовое водоизмещение (displacement_tonnage) должны быть больше нуля.".to_string())
    }

    ///
    /// Возвращает среднюю осадку судна по весовому водоизмещению [м].
    /// Если значение весового водоизмещения выходит за определенный диапазон весового водоизмещения
    /// в гидростатических кривых, возвращает None.
    /// Parameters:
    ///     dispalcement_tonnage - весовое вододоизмещение для которого необходимо определить среднюю осадку.
    #[instrument(skip(self), target = "HydrostaticCurves::mean_draft")]
    pub fn mean_draft(&self, displacement_tonnage: f64) -> Result<Option<f64>, String> {
        if displacement_tonnage > *self.displacement_tonnage.last().unwrap()
            || displacement_tonnage < *self.displacement_tonnage.first().unwrap()
        {
            return Ok(None);
        }
        match self
            .displacement_tonnage
            .custom_binary_search(displacement_tonnage)
        {
            (Some(left_id), Some(right_id)) => {
                let linear_interpolation = LinearInterpolation::new(
                    *self.drafts.get(left_id).unwrap(),
                    *self.drafts.get(right_id).unwrap(),
                    *self.displacement_tonnage.get(left_id).unwrap(),
                    *self.displacement_tonnage.get(right_id).unwrap(),
                );
                Ok(Some(
                    linear_interpolation.interpolated_value(displacement_tonnage)?,
                ))
            }
            (Some(id), None) => Ok(self.drafts.get(id).copied()),
            _ => Ok(None),
        }
    }

    ///
    /// Возвращает данные элементов теоретического чертежа от осадки судна.
    /// Parameters:
    ///     draft - осадка судна,
    ///     type_data - enum HydrostaticTypeData
    #[instrument(skip(self), target = "HydrostaticCurves::get_data_by_draft")]
    pub fn get_data_by_draft(
        &self,
        draft: f64,
        type_data: HydrostaticTypeData,
    ) -> Result<Option<f64>, String> {
        let data = {
            match type_data {
                HydrostaticTypeData::LCB => &self.x_c,
                HydrostaticTypeData::LCF => &self.x_f,
                HydrostaticTypeData::WaterlineArea => &self.waterline_area,
                HydrostaticTypeData::LMR => &self.lmr,
            }
        };
        if draft > *self.drafts.last().unwrap() || draft < *self.drafts.first().unwrap() {
            return Ok(None);
        }
        match self.drafts.custom_binary_search(draft) {
            (Some(left_index), Some(right_index)) => {
                let linear_interpolation = LinearInterpolation::new(
                    *data.get(left_index).unwrap(),
                    *data.get(right_index).unwrap(),
                    *self.drafts.get(left_index).unwrap(),
                    *self.drafts.get(right_index).unwrap(),
                );
                match linear_interpolation.interpolated_value(draft) {
                    Ok(value) => {
                        return Ok(Some(value));
                    }
                    Err(error) => {
                        error!("HydrostaticCurves::get_data_by_draft | {}", error);
                        return Err(error);
                    }
                }
            }
            (Some(index), None) => Ok(data.get(index).copied()),
            _ => Ok(None),
        }
    }

    pub fn max_draft(&self) -> f64 {
        *self.drafts.last().unwrap()
    }

    pub fn min_draft(&self) -> f64 {
        *self.drafts.first().unwrap()
    }

    pub fn max_displacement_tonnage(&self) -> f64 {
        *self.displacement_tonnage.last().unwrap()
    }
}
