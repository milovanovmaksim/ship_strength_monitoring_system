use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium::Spatium}};


/// Load created by the weight of cargo, ballast, tanks, deck cargo, etc.
/// value - load value in tons.
/// center_gravity -  the center gravity of the load relative to the amidships(the middle of a ship).
/// length - load length.
#[derive(Deserialize, Debug)]
pub struct Load {
    value: f64,
    center_gravity: Point,
    length: f64,
}

impl Load {

    ///
    /// Create a new object.
    pub fn new(value: f64, center_gravity: Point, length: f64) -> Self {
        Load { value, center_gravity, length }
    }

    ///
    /// Returns the aft coordinate of the load relative to the midships (middle of the ship)
    fn aft(&self) -> f64 {
        self.longitudinal_center_gravity() - (self.length / 2.0)
    }

    ///
    /// Returns the bow coordinate of the load relative to the amidships(the middle of a ship)
    fn bow(&self) -> f64 {
        self.longitudinal_center_gravity() + (self.length / 2.0)
    }

    ///
    /// Longitudinal center of gravity (LCG)  - the load longitudinal center of gravity relative to the amidships(the middle of a ship)
    fn longitudinal_center_gravity(&self) -> f64 {
        self.center_gravity.x()
    }

    ///
    /// Returns the index of the leftmost spatium that are under the load.
    fn spatium_start_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.aft() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64
    }

    ///
    /// Returns the index of the rightmost spatium that are under the load.
    fn spatium_end_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.bow() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64

    }

    ///
    /// Returns load length.
    pub fn value(&self) -> f64 {
        self.value
    }


    ///
    /// Computes load intensity for a spatium.
    pub fn intensity(&self, ship_demensions: &ShipDimensions) -> Vec<Spatium> {
        let spatium_start_index = self.spatium_start_index(ship_demensions);
        let spatium_end_index = self.spatium_end_index(ship_demensions);
        let mut spatiums = vec![];
        if spatium_end_index < 0 && spatium_start_index < 0 {
            debug!("The deck cargo is outside the leftmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("The deck cargo: {:#?}", self);
        } else if spatium_end_index > ship_demensions.number_spatiums() - 1 && spatium_start_index > ship_demensions.number_spatiums() - 1 {
            debug!("The deck cargo is outside the rightmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("The deck cargo: {:#?}", self);

        } else if spatium_end_index - spatium_start_index > 0 {
            debug!("The deck cargo spreads whitin many spatiums. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("The deck cargo: {:#?}", self);
        } else if spatium_end_index - spatium_start_index == 0 {
            debug!("The deck cargo spreads whitin one spatium. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("The deck cargo: {:#?}", self);
        }
        spatiums


    }
}