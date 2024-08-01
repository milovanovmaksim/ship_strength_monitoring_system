use super::frames::Frames;
use crate::{
    core::linear_interpolation::LinearInterpolation,
    strength::ship::ship_dimensions::ShipDimensions,
};
use tracing::instrument;

///
/// Масштаб Бонжана.
/// Parameters:
///     frames - шпангоуты судна.
///     shipdimensions - размерения судна.
#[derive(Debug)]
pub struct BonjeanScale {
    frames: Frames,
    ship_dimensions: ShipDimensions,
}

impl BonjeanScale {
    pub fn new(frames: Frames, ship_dimensions: ShipDimensions) -> Self {
        BonjeanScale {
            frames,
            ship_dimensions,
        }
    }

    ///
    /// Валидация абсциссы.
    /// Абсцисса не должна выходить за пределы координаты кормы или носа судна.
    /// Parameters:
    ///     abscissa - координата шпангоута относительно центра судна [м],
    #[instrument(err, skip(self), target = "BonjeanScale::validate_abscissa")]
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
    #[instrument(err, skip(self), target = "BonjeanScale::frame_underwater_area")]
    pub fn frame_underwater_area(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        self.validate_abscissa(abscissa)?;
        match self.frames.frame_by_abscissa(abscissa) {
            (Some(frame), None) => frame.area_by_draft(draft),
            (Some(left_frame), Some(right_frame)) => {
                let left_value = left_frame.area_by_draft(draft)?;
                let right_value = right_frame.area_by_draft(draft)?;
                let linear_interpolation = LinearInterpolation::new(
                    left_value,
                    right_value,
                    left_frame.abscissa(),
                    right_frame.abscissa(),
                );
                linear_interpolation.interpolated_value(abscissa)
            }
            _ => {
                unreachable!("Абсцисса лежит в диапазоне между координатой кормы и носа.");
            }
        }
    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Parameters:
    ///     x - координата шпангоута относительно центра судна (абсцисса) [м],
    ///     draft - осадка судна [м].
    #[instrument(err, skip(self), target = "BonjeanScale::frame_underwater_volume")]
    pub fn frame_underwater_volume(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        Ok(self.frame_underwater_area(abscissa, draft)? * self.ship_dimensions.length_spatium())
    }
}
