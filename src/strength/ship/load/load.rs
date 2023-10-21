use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction}};

use crate::strength::ship::load::load_spread::LoadSpread;




/// Load - load created by the weight of cargo, ballast, tanks, deck cargo, etc.
/// value - load component value in tons.
/// center_gravity -  the center gravity of the load component relative to the amidships(the middle of a ship).
/// length - load component length.
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
    /// Returns the coordinate of the start of the load relative to the amidships(the middle of a ship).
    fn load_start_coordinate(&self) -> f64 {
        self.longitudinal_center_gravity() - (self.length / 2.0)
    }

    ///
    /// Returns the coordinate of the end of the load relative to the amidships(the middle of a ship).
    fn load_end_coordinate(&self) -> f64 {
        self.longitudinal_center_gravity() + (self.length / 2.0)
    }

    ///
    /// Longitudinal center of gravity (LCG)  - the load longitudinal center of gravity relative to the amidships(the middle of a ship).
    fn longitudinal_center_gravity(&self) -> f64 {
        self.center_gravity.x()
    }

    ///
    /// Returns the index of the leftmost spatium that are under the load.
    fn spatium_start_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.load_start_coordinate() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64
    }

    ///
    /// Returns the index of the rightmost spatium that are under the load.
    fn spatium_end_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.load_end_coordinate() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64

    }

    ///
    /// Returns load component value in tons.
    pub fn value(&self) -> f64 {
        self.value
    }

    fn dictanse_left(&self, spatium_start_coordinate: f64) -> f64 {
        (self.longitudinal_center_gravity() - spatium_start_coordinate).abs()
    }

    fn distance_right(&self, spatium_end_coordinate: f64) -> f64 {
        (self.longitudinal_center_gravity() - spatium_end_coordinate).abs()
    }

    ///
    /// Computes load intensity.
    pub fn load_intensity(&self, ship_demensions: &ShipDimensions) -> Vec<SpatiumFunction> {
        match self.spread(ship_demensions) {
            LoadSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { self.value * (0.5 + (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let min_intensity = |c_min: f64| { self.value * (0.5 - (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let spatium_start_index = self.spatium_start_index(ship_demensions);
                let spatium_start_coordinate = ship_demensions.spatium_start_coordinate(spatium_start_index as u64);
                let spatium_end_coordinate = ship_demensions.spatium_end_coordinate(spatium_start_index as u64);
                let distance_left = self.dictanse_left(spatium_start_coordinate);
                let distance_right = self.distance_right(spatium_end_coordinate);
                let mut load_component_intensity = vec![];
                if (distance_left > distance_right) && (spatium_start_index as u64 + 1 <= ship_demensions.number_spatiums() - 1) {
                    debug!("Load.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                    let f_x = max_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_ship_dimensions(spatium_start_index as u64, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_ship_dimensions(spatium_start_index as u64 + 1, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                } else if (distance_right > distance_left ) && (spatium_start_index - 1 >= 0) {
                    debug!("Load.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = max_intensity(distance_left);
                     let spatium_function = SpatiumFunction::from_ship_dimensions(spatium_start_index as u64 , ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_ship_dimensions(spatium_start_index as u64 - 1, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                } else {
                    debug!("Load.intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = self.value / ship_demensions.length_spatium();
                    let spatium_function = SpatiumFunction::from_ship_dimensions(spatium_start_index as u64, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                }

            },
            LoadSpread::WithinManySpatiums => {
                let spatium_start_index = self.spatium_start_index(ship_demensions);
                let spatium_end_index = self.spatium_end_index(ship_demensions);
                if self.load_start_coordinate() < ship_demensions.coordinate_aft() {
                    debug!("Часть груза выступает за границу крайнего левого шпангоута. Координата начала груза {}", self.load_start_coordinate());
                    todo!();
                } else if self.load_end_coordinate() > ship_demensions.coordinate_bow() {
                    debug!("Часть груза выступает за границу крайнего правого шпангоута. Координата конца груза {}", self.load_end_coordinate());
                    todo!();

                } else {
                    todo!();
                }

            },
            LoadSpread::OutsideLeftmostFrame => {
                todo!();
            }
            LoadSpread::OutsideRightmostFrame => {
                todo!();
            }
        }
    }

    ///
    /// Determine spread of load.
    /// Returns enum LoadSpread.
    fn spread(&self, ship_demensions: &ShipDimensions) -> LoadSpread {
        let spatium_start_index = self.spatium_start_index(ship_demensions);
        let spatium_end_index = self.spatium_end_index(ship_demensions);
        if self.load_start_coordinate() < ship_demensions.coordinate_aft() && self.load_end_coordinate() < ship_demensions.coordinate_aft() {
            debug!("Load.spread | The load is outside the leftmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load component: {:#?}", self);
            LoadSpread::OutsideLeftmostFrame
        } else if spatium_end_index as u64 > ship_demensions.number_spatiums() - 1 && spatium_start_index as u64 > ship_demensions.number_spatiums() - 1 {
            debug!("Load.spread | The load  is outside the rightmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load : {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadSpread::OutsideRightmostFrame
        } else if spatium_end_index - spatium_start_index > 0 {
            debug!("Load.spread | The load spreads whithin many spatiums. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadSpread::WithinManySpatiums
        } else {
            debug!("Load.spread | The load spreads whithin one spatium. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadSpread::WithinOneSpatium
        }
    }
}