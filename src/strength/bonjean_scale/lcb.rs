use std::rc::Rc;
use tracing::instrument;

use crate::{
    core::linear_interpolation::LinearInterpolation,
    strength::ship::ship_dimensions::ShipDimensions,
};

use super::bonjean_scale::BonjeanScale;

///
/// Абсцисса центра велечины (центр тяжести погруженного объема судна) LCB.
pub struct LCB {
    bonjean_scale: Rc<BonjeanScale>,
    ship_dimensions: ShipDimensions,
}

impl LCB {
    ///
    /// Конструктор.
    pub fn new(bonjean_scale: Rc<BonjeanScale>, ship_dimensions: ShipDimensions) -> Self {
        LCB {
            bonjean_scale,
            ship_dimensions,
        }
    }

    ///
    /// Возвращает абсциссу центра велечины (центр тяжести погруженного объема судна) от осадки. [м]
    /// Parameters:
    ///     aft_draft - осадка кормы [м],
    ///     node_draft - осадка носа [м].
    #[instrument(skip(self), err, target = "LCB::lcb")]
    pub fn lcb(&self, aft_draft: f64, nose_draft: f64) -> Result<f64, String> {
        let length_spatium = self.ship_dimensions.length_spatium();
        let coordinate_aft = self.ship_dimensions.coordinate_aft();
        let mut abscissa = coordinate_aft + length_spatium / 2.0;
        let coordinate_bow = self.ship_dimensions.coordinate_nose();
        let linear_interpolation =
            LinearInterpolation::new(aft_draft, nose_draft, coordinate_aft, coordinate_bow);
        let mut ship_underwater_area = 0.0;
        let mut moment = 0.0;
        for _ in 0..self.ship_dimensions.number_spatiums() {
            let draft = linear_interpolation.interpolated_value(abscissa)?;
            let frame_underwater_area =
                self.bonjean_scale.frame_underwater_area(abscissa, draft)?;
            moment += frame_underwater_area * abscissa;
            ship_underwater_area += frame_underwater_area;
            abscissa += length_spatium
        }
        Ok(moment / ship_underwater_area)
    }
}
