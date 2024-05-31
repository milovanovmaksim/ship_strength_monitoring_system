use log::error;

use crate::{core::linear_interpolation::LinearInterpolation, strength::ship::ship_dimensions::ShipDimensions};
use super::frames::Frames;


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


    ///
    /// Возвращает погруженный объем судна в воду (объемное водоизмещение) для заданных осадок носа и кормы. [м^3]
    /// Parameters:
    ///     aft_draft - осадка кормы [м],
    ///     nose_draft - осадка носа [м].
    pub fn ship_underwater_volume(&self, aft_draft: f64, nose_draft: f64) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        let half_length_spatium = length_spatium / 2.0;
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft + half_length_spatium;
        let coordinate_bow = self.ship_dimensions.coordinate_bow();
        let linear_interpolation = LinearInterpolation::new(aft_draft, nose_draft,
                                                                                 coordinate_aft, coordinate_bow);
        let mut ship_underwater_volume = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            match linear_interpolation.interpolated_value(abscissa) {
                Ok(draft) => {
                    match self.frames.underwater_volume_frame(abscissa, draft, length_spatium) {
                        Ok(underwater_volume_frame) => { ship_underwater_volume += underwater_volume_frame }
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
        Ok(ship_underwater_volume)
    }
}
