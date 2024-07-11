use log::error;

use super::frames::Frames;
use crate::{
    core::linear_interpolation::LinearInterpolation,
    strength::ship::ship_dimensions::ShipDimensions,
};

///
/// Масштаб Бонжана.
/// Parameters:
///     frames - шпангоуты судна.
///     shipdimensions - размерения судна.
pub struct BonjeanScale {
    frames: Frames,
    ship_dimensions: ShipDimensions,
}

impl BonjeanScale {
    pub fn new(frames: Frames, shipdimensions: ShipDimensions) -> Self {
        BonjeanScale {
            frames,
            ship_dimensions: shipdimensions,
        }
    }

    ///
    /// Валидация абсциссы.
    /// Абсцисса не должна выходить за пределы координаты кормы или носа судна.
    /// Parameters:
    ///     abscissa - координата шпангоута относительно центра судна [м],
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
    ///     abscissa - координата шпангоута относительно центра судна [м],
    ///     draft - осадка судна [м].
    pub fn frame_underwater_area(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.validate_abscissa(abscissa) {
            Ok(_) => match self.frames.frame_by_abscissa(abscissa) {
                (Some(frame), None) => match frame.area_by_draft(draft) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        error!("BonjeanScale::frame_underwater_area | error: {}", err);
                        Err(err)
                    }
                },
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
                    let linear_interpolation = LinearInterpolation::new(
                        left_value.unwrap(),
                        right_value.unwrap(),
                        left_frame.abscissa(),
                        right_frame.abscissa(),
                    );
                    match linear_interpolation.interpolated_value(abscissa) {
                        Ok(value) => Ok(value),
                        Err(err) => {
                            error!("BonjeanScale::frame_underwater_area | error: {}", err);
                            Err(err)
                        }
                    }
                }
                _ => {
                    unreachable!("Абсцисса лежит в диапазоне между координатой кормы и носа.");
                }
            },
            Err(err) => {
                error!("BonjeanScale::frame_underwater_area | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Parameters:
    ///     x - координата шпангоута относительно центра судна (абсцисса) [м],
    ///     draft - осадка судна [м].
    pub fn frame_underwater_volume(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        match self.frame_underwater_area(abscissa, draft) {
            Ok(area) => Ok(area * length_spatium),
            Err(err) => {
                error!("BonjeanScale::frame_underwater_volume | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Используется численное интегрирование методом трапеций.
    /// Parameters:
    ///     x - координата шпангоута относительно центра судна (абсцисса) [м],
    ///     draft - осадка судна [м].
    pub fn frame_underwater_volume_trapezoid(
        &self,
        abscissa: f64,
        draft: f64,
    ) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        let area_left_frame = self.frame_underwater_area(abscissa, draft)?;
        let area_right_frame = self.frame_underwater_area(abscissa + length_spatium, draft)?;
        Ok(((area_left_frame + area_right_frame) / 2.0) * length_spatium)
    }
}
