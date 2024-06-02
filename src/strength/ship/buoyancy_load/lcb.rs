use log::error;

use crate::{core::linear_interpolation::LinearInterpolation, strength::ship::ship_dimensions::ShipDimensions};

use super::bonjean_scale::BonjeanScale;


///
/// Абсцисса центра велечины (центр тяжести погруженного объема судна) LCB.
pub struct LCB<'a>{
    bonjean_scale: &'a BonjeanScale<'a>,
    ship_dimensions: ShipDimensions

}

impl<'a> LCB<'a> {

    ///
    /// Конструктор.
    pub fn new(bonjean_scale: &'a BonjeanScale, ship_dimensions: ShipDimensions) -> Self {
        LCB{ bonjean_scale, ship_dimensions }
    }

    ///
    /// Возвращает абсциссу центра велечины (центр тяжести погруженного объема судна).
    /// Parameters:
    ///     aft_draft - осадка кормы [м],
    ///     node_draft - осадка носа [м].
    pub fn lcb(&self, aft_draft: f64, nose_draft: f64) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft + length_spatium / 2.0;
        let coordinate_bow = self.ship_dimensions.coordinate_bow();
        let linear_interpolation = LinearInterpolation::new(aft_draft, nose_draft,
                                                                                 coordinate_aft, coordinate_bow);
        let mut ship_underwater_volume = 0.0;
        let mut moment = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            match linear_interpolation.interpolated_value(abscissa) {
                Ok(draft) => {
                    match self.bonjean_scale.frame_underwater_volume(abscissa, draft) {
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
        Ok(moment / ship_underwater_volume)
    }
}
