
///
/// HydrostaticCurves(гидростатические кривые) - кривые элементов теоретического чертежа, графические зависимости
/// от осадки судна характеристик его плавучести и начальной остойчивости: площади ватерлинии, ее моментов инерции
/// и центра тяжести, водоизмещения, положения центра величины по длине и по высоте, возвышения поперечного
/// и продольного метацентров над килем.
/// Paramenters:
///     drafts: осадка,
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
    pub fn new(drafts: Vec<f64>, displacement_tonnage: Vec<f64>, x_c: Vec<f64>, waterline_area: Vec<f64>, x_f: Vec<f64>) -> Result<Self, String> {
        HydrostaticCurves { drafts, displacement_tonnage, x_c, waterline_area, x_f }.validate_data()
    }

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

    
}