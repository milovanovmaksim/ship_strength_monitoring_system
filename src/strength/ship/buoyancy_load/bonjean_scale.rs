use log::error;

use crate::{core::linear_interpolation::LinearInterpolation, strength::ship::ship_dimensions::ShipDimensions};
use super::frames::Frames;


///
/// Абсцисса центра велечины (центр тяжести погруженного объема судна)
#[derive(Clone, Copy)]
pub struct LCB(pub f64);


///
/// О0бъемное водоизмещение судна
#[derive(Clone, Copy)]
pub struct Displacement(pub f64);


///
/// Масштаб Бонжана.
/// Parameters:
///     frames - шпангоуты судна.
///     shipdimensions - размерения судна.
pub(crate) struct BonjeanScale {
    frames: Frames,
    ship_dimensions: ShipDimensions,
}

impl BonjeanScale {
    pub fn new(frames: Frames, shipdimensions: ShipDimensions) -> Self {
        BonjeanScale { frames, ship_dimensions: shipdimensions }
    }


    ///
    /// Валидация абсциссы.
    /// Абсцисса не должна выходить за пределы координаты кормы или носа судна.
    /// Parameters:
    ///     abscissa - координата шпангоута относительно центра корабля [м],
    fn validate_abscissa(&self, abscissa: f64) -> Result<(), String> {
        if abscissa < self.frames.first().abscissa() {
            return Err(format!("Абсцисса вышла за пределы координаты кормы судна. Координа кормы: {}. Передано значение: {}",
                self.frames.first().abscissa(), abscissa));
        }
        if abscissa > self.frames.last().abscissa() {
            return Err(format!("Абсцисса вышла за пределы координаты носа судна. Координа носа: {}. Передано значение: {}",
                self.frames.last().abscissa(), abscissa));
        }
        Ok(())
    }


    ///
    /// Возвращает погруженную площадь шпангоута для заданной осадки и абсциссы. [м^2]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно инерполирует
    /// площадь шпангоутов, имея в распоряжение площадь двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     abscissa - координата шпангоута относительно центра корабля [м],
    ///     draft - осадка корабля [м].
    fn frame_underwater_area(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.validate_abscissa(abscissa) {
            Ok(_) => {
                match self.frames.frame_by_abscissa(abscissa) {
                    (Some(frame), None) => {
                        match frame.area_by_draft(draft) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("BonjeanScale::frame_underwater_area | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    (Some(left_frame), Some(right_frame)) => {
                        let left_value = left_frame.area_by_draft(draft);
                        if let Err(err) = left_value {
                            error!("BonjeanScale::frame_underwater_area | error: {}", err);
                            return Err(err);
                        }
                        let right_value = right_frame.area_by_draft(draft);
                        if let Err(err) = right_value {
                            error!("BonjeanScale::frame_underwater_area | error: {}", err);
                            return Err(err);
                        }
                        let linear_interpolation = LinearInterpolation::new(left_value.unwrap(), right_value.unwrap(),
                            left_frame.abscissa(), right_frame.abscissa());
                        match linear_interpolation.interpolated_value(abscissa) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("BonjeanScale::frame_underwater_area | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    _ => { unreachable!("Абсцисса лежит в диапазоне между координатой кормы и носа."); }
                }
            }
            Err(err) => {
                error!("BonjeanScale::frame_underwater_area | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    fn frame_underwater_volume(&self, abscissa: f64, draft: f64,) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        match self.frame_underwater_area(abscissa, draft) {
            Ok(area) => { Ok(area * length_spatium) }
            Err(err) => {
                error!("BonjeanScale::frame_underwater_volume | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Вычисляет данные масштаба Бонжана.
    pub fn bonjean_scale(&mut self, aft_draft: f64, nose_draft: f64) -> Result<(LCB, Displacement), String>{
        let length_spatium = self.ship_dimensions.length_spatium();
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft;
        let coordinate_bow = self.ship_dimensions.coordinate_bow();
        let linear_interpolation = LinearInterpolation::new(aft_draft, nose_draft,
                                                                                coordinate_aft, coordinate_bow);
        let mut ship_underwater_volume = 0.0;
        let mut moment = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            match linear_interpolation.interpolated_value(abscissa) {
                Ok(draft) => {
                    match self.frame_underwater_volume(abscissa, draft) {
                        Ok(frame_underwater_volume) => {
                            moment += frame_underwater_volume * abscissa;
                            ship_underwater_volume += frame_underwater_volume;
                        }
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
            abscissa += length_spatium
        }
        Ok((LCB((moment / ship_underwater_volume)), Displacement(ship_underwater_volume)))
    }

}
