use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium::Spatium}};


///
/// DeckCargo - Goods shipped on the deck of a ship rather than in its holds.
/// Various types of deck cargoes:
    ///  - Timber deck cargoes.
    ///  - Containers.
    ///  - Vehicles.
    ///  - Livestock.
    ///  - Iron and steel pipes or girders.
    ///  - Dangerous goods.
    ///  - Heavy lifts and unusually shaped goods such as locomotives,
    ///    yachts and small launches, large tanks or pressure vessels and other such machinery may also be shipped.
/// weight - deck cargo weight.
/// center_gravity -  the center gravity of the deck cargo relative to the amidships(the middle of a ship).
/// length - deck cargo length.
#[derive(Deserialize, Debug)]
pub struct DeckCargo {
    weight: f64,
    center_gravity: Point,
    length: f64,
}

impl DeckCargo {

    pub fn new(value: f64, center_gravity: Point, length: f64) -> Self {
        DeckCargo { weight: value, center_gravity, length }
    }

    ///
    /// Returns the aft coordinate of the deck cargo relative to the midships (middle of the ship)
    fn aft(&self) -> f64 {
        self.longitudinal_center_gravity() - (self.length / 2.0)
    }

    ///
    /// Returns the bow coordinate of the deck cargo relative to the amidships(the middle of a ship)
    fn bow(&self) -> f64 {
        self.longitudinal_center_gravity() + (self.length / 2.0)
    }

    ///
    /// Longitudinal center of gravity (LCG)  - the deck cargo longitudinal center of gravity relative to the amidships(the middle of a ship)
    fn longitudinal_center_gravity(&self) -> f64 {
        self.center_gravity.x()
    }

    ///
    /// Returns the index of the leftmost spatium that are under the load created by the deck cargo weight.
    fn spatium_start_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.aft() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64
    }

    ///
    /// Returns the index of the rightmost spatium that are under the load created by the deck cargo weight.
    fn spatium_end_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.bow() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64

    }

    ///
    /// Retuns deck cargo weight
    pub fn weight(&self) -> f64 {
        self.weight
    }


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