use log::error;

use crate::{core::linear_interpolation::LinearInterpolation, strength::ship::{buoyancy_load::bonjean_scale::BonjeanScale, ship_dimensions::ShipDimensions}};


///
/// Объемное водоизмещение судна.
pub struct Displacement<'a> {
    bonjean_scale: &'a BonjeanScale<'a>,
    ship_dimensions: ShipDimensions
}


impl<'a> Displacement<'a> {

    ///
    /// Конструктор.
    pub fn new(bonjean_scale: &'a BonjeanScale, ship_dimensions: ShipDimensions) -> Self {
        Displacement { bonjean_scale, ship_dimensions }
    }

    ///
    /// Возвращает объемное водоизмещение судна от осадки. [м^3]
    pub fn displacement(&self, aft_draft: f64, nose_draft: f64) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft + length_spatium / 2.0;
        let coordinate_bow = self.ship_dimensions.coordinate_bow();
        let linear_interpolation = LinearInterpolation::new(aft_draft, nose_draft,
                                                                                 coordinate_aft, coordinate_bow);
        let mut ship_underwater_volume = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            match linear_interpolation.interpolated_value(abscissa) {
                Ok(draft) => {
                    match self.bonjean_scale.frame_underwater_volume(abscissa, draft) {
                        Ok(frame_underwater_volume) => {
                            ship_underwater_volume += frame_underwater_volume;
                        }
                        Err(err) => {
                            error!("LCB::lcb | error: {}", err);
                            return Err(err);
                        }
                    }
                }
                Err(err) => {
                    error!("LCB::lcb | error: {}", err);
                    return Err(err);
                }
            }
            abscissa += length_spatium
        }
        Ok(ship_underwater_volume)
    }
}