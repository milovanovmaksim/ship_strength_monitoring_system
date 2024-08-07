use std::rc::Rc;
use tracing::instrument;

use crate::{
    core::{linear_interpolation::LinearInterpolation, water_density::WaterDensity},
    strength::{bonjean_scale::bonjean_scale::BonjeanScale, ship::ship_dimensions::ShipDimensions},
};

///
/// Объемное водоизмещение судна.
pub struct Displacement {
    bonjean_scale: Rc<BonjeanScale>,
    ship_dimensions: ShipDimensions,
    water_density: WaterDensity,
}

impl Displacement {
    ///
    /// Основной конструктор.
    pub fn new(
        bonjean_scale: Rc<BonjeanScale>,
        ship_dimensions: ShipDimensions,
        water_density: WaterDensity,
    ) -> Self {
        Displacement {
            bonjean_scale,
            ship_dimensions,
            water_density,
        }
    }

    ///
    /// Возвращает объемное водоизмещение судна от осадки. [м^3]
    #[instrument(skip(self), err, target = "Displacement::displacement_by_drafts")]
    pub fn displacement_by_drafts(&self, aft_draft: f64, nose_draft: f64) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft + length_spatium / 2.0;
        let coordinate_bow = self.ship_dimensions.coordinate_nose();
        let linear_interpolation =
            LinearInterpolation::new(aft_draft, nose_draft, coordinate_aft, coordinate_bow);
        let mut ship_underwater_volume = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            let draft = linear_interpolation.interpolated_value(abscissa)?;
            let frame_underwater_volume = self
                .bonjean_scale
                .frame_underwater_volume(abscissa, draft)?;
            ship_underwater_volume += frame_underwater_volume;
            abscissa += length_spatium
        }
        Ok(ship_underwater_volume)
    }

    ///
    /// Возвращает объемное водоизмещение судна от массы. [м^3]
    pub fn displacement_by_mass(&self, massa: f64) -> f64 {
        massa / self.water_density
    }
}
