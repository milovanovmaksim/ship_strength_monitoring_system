use log::error;

use crate::{core::linear_interpolation::LinearInterpolation, strength::ship::ship_dimensions::ShipDimensions};
use super::{bonjean_scale_data_type::BonjeanScaleDataType, frames::Frames};


///
/// Масштаб Бонжана.
/// Parameters:
///     frames - шпангоуты судна.
///     shipdimensions - размерения судна.
pub(crate) struct BonjeanScale {
    frames: Frames,
    ship_dimensions: ShipDimensions
}

impl BonjeanScale {
    pub fn new(frames: Frames, shipdimensions: ShipDimensions) -> Self {
        BonjeanScale { frames, ship_dimensions: shipdimensions }
    }


    fn validate_abscissa(&self, abscissa: f64) -> Result<(), String> {
        if abscissa < self.ship_dimensions.coordinate_aft() {
            return Err(format!("Абсцисса вышла за пределы координаты кормы судна. Координа кормы: {}. Передано значение: {}",
                self.ship_dimensions.coordinate_aft(), abscissa));
        }
        if abscissa > self.ship_dimensions.coordinate_bow() {
            return Err(format!("Абсцисса вышла за пределы координаты носа судна. Координа носа: {}. Передано значение: {}",
                self.ship_dimensions.coordinate_bow(), abscissa));
        }
        Ok(())
    }

    fn bonjean_scale_data(&self, abscissa: f64, draft: f64, type_data: BonjeanScaleDataType) -> Result<f64, String> {
        match self.validate_abscissa(abscissa) {
            Ok(_) => {
                match self.frames.frame_id_by_abscissa(abscissa) {
                    (Some(index), None) => {
                        let frame = self.frames.get(index).unwrap();
                        match frame.data_by_draft(draft, type_data) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    (Some(left_index), Some(right_index)) => {
                        let left_frame = self.frames.get(left_index).unwrap();
                        let right_frame  = self.frames.get(right_index).unwrap();
                        let left_value = left_frame.data_by_draft(draft, type_data);
                        if let Err(err) = left_value {
                            error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                            return Err(err);
                        }
                        let right_value = right_frame.data_by_draft(draft, type_data);
                        if let Err(err) = right_value {
                            error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                            return Err(err);
                        }
                        let linear_interpolation = LinearInterpolation::new(left_value.unwrap(), right_value.unwrap(),
                            left_frame.abscissa(), right_frame.abscissa());
                        match linear_interpolation.interpolated_value(abscissa) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("BonjeanScale::bonjean_scale_data | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    _ => { unreachable!("Абсцисса лежит в диапазоне между координатой кормы и носа."); }
                }
            }
            Err(err) => {
                error!("BonjeanScale::bonjean_scale_data | error: {}", err);
                Err(err)
            }
        }
    }


    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно интерполирует
    /// объем шпангоутов, имея в распоряжение объем двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_volume_frame(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.bonjean_scale_data(abscissa, draft, BonjeanScaleDataType::Volume) {
            Ok(value) => { Ok(value) }
            Err(err) => {
                error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Возвращает погруженную площадь шпангоута для заданной осадки и абсциссы. [м^2]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно инерполирует
    /// площадь шпангоутов, имея в распоряжение площадь двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_area_frame(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.bonjean_scale_data(abscissa, draft, BonjeanScaleDataType::Area) {
            Ok(value) => { Ok(value) }
            Err(err) => {
                error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                Err(err)
            }
        }
    }


    ///
    /// Возвращает погруженный объем судна в воду (объемное водоизмещение) для заданных осадок носа и кормы. [м^3]
    /// Parameters:
    ///     aft_draft - осадка кормы [м],
    ///     nose_draft - осадка носа [м].
    pub fn ship_underwater_volume(&self, aft_draft: f64, nose_draft: f64) -> Result<f64, String> {
        let half_length_spatium = self.ship_dimensions.length_spatium() / 2.0;
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft + half_length_spatium;
        let coordinate_bow = self.ship_dimensions.coordinate_bow();
        let linear_interpolation = LinearInterpolation::new(aft_draft, nose_draft,
                                                                                 coordinate_aft, coordinate_bow);
        let mut ship_underwater_volume = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            match linear_interpolation.interpolated_value(abscissa) {
                Ok(draft) => {
                    match self.underwater_volume_frame(abscissa, draft) {
                        Ok(underwater_volume_frame) => { ship_underwater_volume += underwater_volume_frame; }
                        Err(err) => {
                            error!("BonjeanScale::ship_underwater_volume | error: {}", err);
                            return Err(err);
                        }
                    }
                }
                Err(err) => {
                    error!("BonjeanScale::ship_underwater_volume | error: {}", err);
                    return Err(err);
                }
            }
            abscissa += self.ship_dimensions.length_spatium();
        }
        Ok(ship_underwater_volume)
    }
}
