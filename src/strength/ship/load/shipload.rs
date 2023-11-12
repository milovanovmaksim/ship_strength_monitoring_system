use log::debug;
use serde::Deserialize;
use crate::core::round::Round;
use crate::{core::point::Point, strength::ship::ship_dimensions::ShipDimensions};

use crate::strength::ship::load::load_spread::LoadSpread;





///
/// SpatiumLoad - load acting on one spatium.
/// value - load value in tons.
/// center_gravity -  the center gravity of the load relative to the amidships(the middle of a ship).
/// length - load length.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Shipload {
    value: f64,
    center_gravity: Point,
    length: f64,
}

impl Shipload {

    ///
    /// Create a new object.
    pub fn new(value: f64, center_gravity: Point, length: f64) -> Self {
        Shipload { value, center_gravity, length }
    }

    ///
    /// Return the coordinate of the start of the load relative to the amidships(the middle of a ship).
    pub fn load_start_coordinate(&self) -> f64 {
        self.longitudinal_center_gravity() - (self.length / 2.0)
    }

    ///
    /// Return the coordinate of the end of the load relative to the amidships(the middle of a ship).
    pub fn load_end_coordinate(&self) -> f64 {
        self.longitudinal_center_gravity() + (self.length / 2.0)
    }

    ///
    /// Longitudinal center of gravity (LCG)  - the load longitudinal center of gravity relative to the amidships(the middle of a ship).
    pub fn longitudinal_center_gravity(&self) -> f64 {
        self.center_gravity.x
    }

    ///
    /// Return shipload value in tons.
    pub fn value(&self) -> f64 {
        self.value
    }

    ///
    /// Distances from LCG of the shipload to left and right frames.
    pub fn distances_to_frames(&self, ship_dimensions: &ShipDimensions) -> (f64, f64) {
        let spatium_start_index = ship_dimensions.spatium_index_by_coordinate(self.load_start_coordinate());
        let spatium_start_coordinate = ship_dimensions.spatium_start_coordinate(spatium_start_index);
        let spatium_end_coordinate = ship_dimensions.spatium_end_coordinate(spatium_start_index);
        let distance_left = (self.longitudinal_center_gravity().abs() - spatium_start_coordinate.abs()).abs().my_round(2);
        let distance_right = (self.longitudinal_center_gravity().abs() - spatium_end_coordinate.abs()).abs().my_round(2);
        (distance_left, distance_right)
    }

    ///
    /// Pinch off the shipload.
    /// Params:
        /// load_start_coordinate - shipload start coordinate.
        /// load_end_coordinate - shipload end coordinate.
    /// Return: Shipload.
    pub fn shared_shipload(&self, load_start_coordinate: f64, load_end_coordinate: f64) -> Shipload {
        let load_length = (load_end_coordinate - load_start_coordinate).abs().my_round(2);
        debug!("{}", load_length);
        let load_value = (load_length / self.length) * self.value;
        let x = load_start_coordinate + (load_length) / 2.0;
        let center_gravity = Point::new(x, self.center_gravity.y, self.center_gravity.z);
        Shipload::new(load_value, center_gravity, load_length)
    }

    ///
    /// Determine spread of th shipload.
    pub fn spread(&self, ship_dimensions: &ShipDimensions) -> LoadSpread {
        let spatium_start_index = ship_dimensions.spatium_index_by_coordinate(self.load_start_coordinate());
        let spatium_end_index = ship_dimensions.spatium_index_by_coordinate(self.load_end_coordinate());
        let spatium_start_coordinate = ship_dimensions.spatium_start_coordinate(spatium_start_index);
        if self.load_start_coordinate() < ship_dimensions.coordinate_aft() && self.load_end_coordinate() <= ship_dimensions.coordinate_aft() {
            debug!("Load.spread | The load is outside the leftmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}", self);
            LoadSpread::OutsideLeftmostFrame
        } else if self.load_start_coordinate() >= ship_dimensions.coordinate_bow() && self.load_end_coordinate() > ship_dimensions.coordinate_bow()  {
            debug!("Load.spread | The load  is outside the rightmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_dimensions);
            LoadSpread::OutsideRightmostFrame
        } else if self.load_start_coordinate().my_round(2) >= spatium_start_coordinate.my_round(2) && self.load_end_coordinate().my_round(2) <= (spatium_start_coordinate + ship_dimensions.length_spatium()).my_round(2) {
            debug!("Load.spread | The load spreads whithin one spatium. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_dimensions);
            LoadSpread::WithinOneSpatium
        } else {
            debug!("Load.spread | The load spreads whithin many spatiums. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_dimensions);
            LoadSpread::WithinManySpatiums
        }
    }
}