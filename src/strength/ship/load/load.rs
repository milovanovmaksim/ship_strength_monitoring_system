use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction}};

use crate::strength::ship::load::load_spread::LoadSpread;




/// Load - load created by the weight of cargo, ballast, tanks, deck cargo, etc.
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
    fn spatium_start_index(&self, ship_demensions: &ShipDimensions) -> u64 {
        let x = self.load_start_coordinate();
        ship_demensions.spatium_index_by_coordinate(x)
    }

    ///
    /// Returns the index of the rightmost spatium that are under the load.
    fn spatium_end_index(&self, ship_demensions: &ShipDimensions) -> u64 {
        let x = self.load_end_coordinate();
        ship_demensions.spatium_index_by_coordinate(x)

    }

    ///
    /// Returns load component value in tons.
    pub fn value(&self) -> f64 {
        self.value
    }

    fn distance(&self, ship_demensions: &ShipDimensions) -> (f64, f64) {
        let spatium_start_index = self.spatium_start_index(ship_demensions);
        let spatium_start_coordinate = ship_demensions.spatium_start_coordinate(spatium_start_index);
        let spatium_end_coordinate = ship_demensions.spatium_end_coordinate(spatium_start_index);
        let distance_left = (self.longitudinal_center_gravity().abs() - spatium_start_coordinate.abs()).abs();
        let distance_right = (self.longitudinal_center_gravity().abs() - spatium_end_coordinate.abs()).abs();
        (distance_left, distance_right)
    }

    fn separated_load(&self, load_start_coordinate: f64, load_end_coordinate: f64) -> Load {
        let load_length = (load_start_coordinate.abs() - load_end_coordinate.abs()).abs();
        let longitudinal_center_gravity = load_start_coordinate + (load_length / 2.0);
        let center_gravity = Point::new(longitudinal_center_gravity, self.center_gravity.y(), self.center_gravity.z());
        let load_value = (load_length / self.length) * self.value;
        Load::new(load_value, center_gravity, load_length)

    }

    ///
    /// Computes load intensity.
    pub fn load_intensity(&self, ship_demensions: &ShipDimensions) -> Vec<SpatiumFunction> {
        match self.spread(ship_demensions) {
            LoadSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { self.value * (0.5 + (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let min_intensity = |c_min: f64| { self.value * (0.5 - (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let (distance_left, distance_right) = self.distance(ship_demensions);
                let spatium_start_index = self.spatium_start_index(ship_demensions);
                let mut load_component_intensity = vec![];
                if (distance_left > distance_right) && (self.longitudinal_center_gravity() + ship_demensions.length_spatium() < ship_demensions.coordinate_bow()) {
                    debug!("Load.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                    let f_x = max_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index + 1, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                } else if (distance_right > distance_left ) && (self.longitudinal_center_gravity() - ship_demensions.length_spatium()) > ship_demensions.coordinate_aft() {
                    debug!("Load.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = max_intensity(distance_left);
                     let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index - 1, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                } else {
                    debug!("Load.intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = self.value / ship_demensions.length_spatium();
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                }
            },
            LoadSpread::WithinManySpatiums => {
                let x_1 = self.load_start_coordinate();
                let x_4 = self.load_end_coordinate();
                let spatium_start_index = self.spatium_start_index(ship_demensions);
                let saptium_end_index = self.spatium_end_index(ship_demensions);
                let x_2 = ship_demensions.spatium_end_coordinate(spatium_start_index);
                let x_3 = ship_demensions.spatium_start_coordinate(saptium_end_index);
                let mut load_intensity: Vec<SpatiumFunction> = vec![];
                if (x_1.abs() - x_2.abs()).abs() > 0.0 {
                    let load = self.separated_load(x_1, x_2);
                    let spatium_functions = load.load_intensity(ship_demensions);
                    load_intensity.extend(spatium_functions);
                } else if (x_4.abs() - x_3.abs()).abs() > 0.0 {
                    let load = self.separated_load(x_3, x_4);
                    let spatium_functions = load.load_intensity(ship_demensions);
                    load_intensity.extend(spatium_functions);
                }
                let mut load_start_coordinate = x_2;
                let mut load_end_coordinate = x_2 + ship_demensions.length_spatium();
                let number_whole_spatiums_under_load = ((x_2 - x_3).abs() / ship_demensions.length_spatium()) as i64;
                for _ in 0..number_whole_spatiums_under_load {
                    let load = self.separated_load(load_start_coordinate, load_end_coordinate);
                    let spatium_functions = load.load_intensity(ship_demensions);
                    load_intensity.extend(spatium_functions);
                    load_start_coordinate += ship_demensions.length_spatium();
                    load_end_coordinate += ship_demensions.length_spatium();
                }
                debug!("Saptiums are under the load {:#?}", load_intensity);
                load_intensity
            },
            LoadSpread::OutsideLeftmostFrame => {
                let leftmost_spatium_id = 0;
                let f_x = ((1.5 + (self.longitudinal_center_gravity() / ship_demensions.length_spatium())) * self.value) / ship_demensions.length_spatium();
                let mut load_intensity: Vec<SpatiumFunction> = vec![];
                let spatium_function = SpatiumFunction::from_id(leftmost_spatium_id, ship_demensions, f_x, f_x);
                load_intensity.push(spatium_function);

                let f_x = -((0.5 + (self.longitudinal_center_gravity() / ship_demensions.length_spatium())) * self.value) / ship_demensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(leftmost_spatium_id + 1, ship_demensions, f_x, f_x);
                load_intensity.push(spatium_function);
                debug!("Saptiums are under the load {:#?}", load_intensity);
                load_intensity
            }
            LoadSpread::OutsideRightmostFrame => {
                let rightmost_spatium_id = ship_demensions.number_spatiums() - 1;
                let f_x = ((1.5 + (self.longitudinal_center_gravity() / ship_demensions.length_spatium())) * self.value) / ship_demensions.length_spatium();
                let mut load_intensity: Vec<SpatiumFunction> = vec![];
                let spatium_function = SpatiumFunction::from_id(rightmost_spatium_id, ship_demensions, f_x, f_x);
                load_intensity.push(spatium_function);

                let f_x = -((0.5 + (self.longitudinal_center_gravity() / ship_demensions.length_spatium())) * self.value) / ship_demensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(rightmost_spatium_id - 1, ship_demensions, f_x, f_x);
                load_intensity.push(spatium_function);
                debug!("Saptiums are under the load {:#?}", load_intensity);
                load_intensity
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
            debug!("Load.spread | The load: {:#?}", self);
            LoadSpread::OutsideLeftmostFrame
        } else if self.load_start_coordinate() > ship_demensions.coordinate_bow() && self.load_end_coordinate() > ship_demensions.coordinate_bow()  {
            debug!("Load.spread | The load  is outside the rightmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("Load.spread | The load: {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadSpread::OutsideRightmostFrame
        } else if self.load_end_coordinate() - self.load_start_coordinate() > ship_demensions.length_spatium() {
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