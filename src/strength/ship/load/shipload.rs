use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction}};

use crate::strength::ship::load::load_spread::LoadSpread;

use super::load_sharing::LoadSharing;



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
    fn longitudinal_center_gravity(&self) -> f64 {
        self.center_gravity.x
    }

    ///
    /// Return the index of the leftmost spatium that are under the load.
    fn spatium_start_index(&self, ship_dimensions: &ShipDimensions) -> i64 {
        let x = self.load_start_coordinate();
        ship_dimensions.spatium_index_by_coordinate(x)
    }

    ///
    /// Return the index of the rightmost spatium that are under the load.
    fn spatium_end_index(&self, ship_dimensions: &ShipDimensions) -> i64 {
        let x = self.load_end_coordinate();
        ship_dimensions.spatium_index_by_coordinate(x)

    }

    ///
    /// Return shipload value in tons.
    pub fn value(&self) -> f64 {
        self.value
    }

    ///
    /// The load length.
    pub fn length(&self) -> f64 {
        self.length
    }

    ///
    /// Distances from LCG of the shipload to left and right frames.
    fn distances_to_frames(&self, ship_demensions: &ShipDimensions) -> (f64, f64) {
        let spatium_start_index = self.spatium_start_index(ship_demensions);
        let spatium_start_coordinate = ship_demensions.spatium_start_coordinate(spatium_start_index);
        let spatium_end_coordinate = ship_demensions.spatium_end_coordinate(spatium_start_index);
        let distance_left = (self.longitudinal_center_gravity().abs() - spatium_start_coordinate.abs()).abs();
        let distance_right = (self.longitudinal_center_gravity().abs() - spatium_end_coordinate.abs()).abs();
        (distance_left, distance_right)
    }

    ///
    /// Pinch off the shipload.
    /// Params:
        /// load_start_coordinate - shipload start coordinate.
        /// load_end_coordinate - shipload end coordinate.
    /// Return: Shipload.
    pub fn shared_shipload(&self, load_start_coordinate: f64, load_end_coordinate: f64) -> Shipload {
        let load_length = (load_start_coordinate.abs() - load_end_coordinate.abs()).abs();
        let load_value = (load_length / self.length) * self.value;
        Shipload::new(load_value, self.center_gravity, load_length)
    }

    ///
    /// Compute the load intensity.
    pub fn shipload_intensity(&self, ship_dimensions: &ShipDimensions) -> Vec<SpatiumFunction> {
        match self.spread(&ship_dimensions) {
            LoadSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { self.value * (0.5 + (c_min / ship_dimensions.length_spatium())) / ship_dimensions.length_spatium() };
                let min_intensity = |c_min: f64| { self.value * (0.5 - (c_min / ship_dimensions.length_spatium())) / ship_dimensions.length_spatium() };
                let (distance_left, distance_right) = self.distances_to_frames(ship_dimensions);
                let spatium_start_index = self.spatium_start_index(ship_dimensions);
                let mut shipload_intensity = vec![];
                if (distance_left > distance_right) && (self.longitudinal_center_gravity() + ship_dimensions.length_spatium() < ship_dimensions.coordinate_bow()) {
                    debug!("Load.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                    let f_x = max_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index + 1, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", shipload_intensity);
                    shipload_intensity
                } else if (distance_right > distance_left ) && (self.longitudinal_center_gravity() - ship_dimensions.length_spatium()) > ship_dimensions.coordinate_aft() {
                    debug!("Load.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = max_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index - 1, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", shipload_intensity);
                    shipload_intensity
                } else {
                    debug!("Load.intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = self.value / ship_dimensions.length_spatium();
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", shipload_intensity);
                    shipload_intensity
                }
            },
            LoadSpread::OutsideLeftmostFrame | LoadSpread::OutsideRightmostFrame => {
                let (spatium_id, next_spatium_id, distance) = {
                    if self.load_start_coordinate() < ship_dimensions.coordinate_aft() && self.load_end_coordinate() <= ship_dimensions.coordinate_aft() {
                        let distance = (ship_dimensions.coordinate_aft().abs() - self.longitudinal_center_gravity().abs()).abs();
                        (0, 1, distance)
                    } else {
                        let rightmost_spatium_id = ship_dimensions.number_spatiums() - 1;
                        let distance = (ship_dimensions.coordinate_bow().abs() - self.longitudinal_center_gravity().abs()).abs();
                        (rightmost_spatium_id, rightmost_spatium_id - 1, distance)
                    }
                };
                let f_x = ((1.5 + (distance / ship_dimensions.length_spatium())) * self.value) / ship_dimensions.length_spatium();
                let mut shipload_intensity: Vec<SpatiumFunction> = vec![];
                let spatium_function = SpatiumFunction::from_id(spatium_id as i64, ship_dimensions, f_x, f_x);
                shipload_intensity.push(spatium_function);

                let f_x = -((0.5 + (distance / ship_dimensions.length_spatium())) * self.value) / ship_dimensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(next_spatium_id as i64, ship_dimensions, f_x, f_x);
                shipload_intensity.push(spatium_function);
                debug!("Saptiums are under the load {:#?}", shipload_intensity);
                shipload_intensity
            },
            LoadSpread::WithinManySpatiums => {
                let load_sharing = LoadSharing::new(ship_dimensions, &self);
                let shiploads = load_sharing.shared_loads();
                let mut shipload_intensity: Vec<SpatiumFunction> = vec![];
                for shipload in shiploads.iter() {
                    let spatium_functions = shipload.shipload_intensity(ship_dimensions);
                    shipload_intensity.extend(spatium_functions);
                }
                shipload_intensity
            }
        }
    }

    ///
    /// Determine spread of th shipload.
    pub fn spread(&self, ship_dimensions: &ShipDimensions) -> LoadSpread {
        let spatium_start_index = self.spatium_start_index(ship_dimensions);
        let spatium_end_index = self.spatium_end_index(ship_dimensions);
        let spatium_start_coordinate = ship_dimensions.spatium_start_coordinate(spatium_start_index);
        if self.load_start_coordinate() < ship_dimensions.coordinate_aft() && self.load_end_coordinate() <= ship_dimensions.coordinate_aft() {
            debug!("Load.spread | The load is outside the leftmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}", self);
            LoadSpread::OutsideLeftmostFrame
        } else if self.load_start_coordinate() >= ship_dimensions.coordinate_bow() && self.load_end_coordinate() > ship_dimensions.coordinate_bow()  {
            debug!("Load.spread | The load  is outside the rightmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_dimensions);
            LoadSpread::OutsideRightmostFrame
        } else if self.load_start_coordinate() >= spatium_start_coordinate && self.load_end_coordinate() <= spatium_start_coordinate + ship_dimensions.length_spatium() {
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