use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction}};

use crate::strength::ship::load::load_component_spread::LoadComponentSpread;




/// LoadComponent - load created by the weight of cargo, ballast, tanks, deck cargo, etc.
/// value - load component value in tons.
/// center_gravity -  the center gravity of the load component relative to the amidships(the middle of a ship).
/// length - load component length.
#[derive(Deserialize, Debug)]
pub struct LoadComponent {
    value: f64,
    center_gravity: Point,
    length: f64,
}

impl LoadComponent {

    ///
    /// Create a new object.
    pub fn new(value: f64, center_gravity: Point, length: f64) -> Self {
        LoadComponent { value, center_gravity, length }
    }

    ///
    /// Returns the coordinate of the start of the load component relative to the amidships(the middle of a ship).
    fn aft(&self) -> f64 {
        self.longitudinal_center_gravity() - (self.length / 2.0)
    }

    ///
    /// Returns the coordinate of the start of the load component relative to the amidships(the middle of a ship).
    fn bow(&self) -> f64 {
        self.longitudinal_center_gravity() + (self.length / 2.0)
    }

    ///
    /// Longitudinal center of gravity (LCG)  - the load component longitudinal center of gravity relative to the amidships(the middle of a ship).
    fn longitudinal_center_gravity(&self) -> f64 {
        self.center_gravity.x()
    }

    ///
    /// Returns the index of the leftmost spatium that are under the load component.
    fn spatium_start_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.aft() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64
    }

    ///
    /// Returns the index of the rightmost spatium that are under the load component
    fn spatium_end_index(&self, ship_demensions: &ShipDimensions) -> i64 {
        ((self.bow() / ship_demensions.length_spatium()) + (ship_demensions.number_spatiums()) as f64 / 2.0) as i64

    }

    fn spatium_start_coordinate(&self, id: i64, ship_demensions: &ShipDimensions) -> f64 {
        id as f64 * ship_demensions.length_spatium() - (ship_demensions.length_between_perpendiculars() / 2.0)
    }

    fn spatium_end_coordinate(&self, id: i64, ship_demensions: &ShipDimensions) -> f64 {
        self.spatium_start_coordinate(id, ship_demensions) + ship_demensions.length_spatium()

    }

    ///
    /// Returns load component value in tons.
    pub fn value(&self) -> f64 {
        self.value
    }

    fn distances(&self, ship_demensions: &ShipDimensions) -> (f64, f64) {
        let spatium_index = self.spatium_start_index(ship_demensions);
        let spatium_start_coordinate = self.spatium_start_coordinate(spatium_index, ship_demensions);
        let spatium_end_coordinate = self.spatium_end_coordinate(spatium_index, ship_demensions);
        let distance_left = (self.longitudinal_center_gravity() - spatium_start_coordinate).abs();
        let distance_right = (self.longitudinal_center_gravity() - spatium_end_coordinate).abs();
        (distance_left, distance_right)
    }

    fn load_intensity_for_spatium(&self, id: i64, ship_demensions: &ShipDimensions, f_x1: f64, f_x2: f64) -> SpatiumFunction {
        let spatium_start_coordinate = self.spatium_start_coordinate(id, ship_demensions);
        let spatium_end_coordinate = self.spatium_end_coordinate(id, ship_demensions);
        SpatiumFunction::new(spatium_start_coordinate, spatium_end_coordinate, f_x1, f_x2)
    }

    ///
    /// Computes load component intensity.
    pub fn load_component_intensity(&self, ship_demensions: &ShipDimensions) -> Vec<SpatiumFunction> {
        match self.spread(ship_demensions) {
            LoadComponentSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { self.value * (0.5 + (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let min_intensity = |c_min: f64| { self.value * (0.5 - (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let spatium_start_index = self.spatium_start_index(ship_demensions);
                let (distance_left, distance_right) = self.distances(ship_demensions);
                let mut load_component_intensity = vec![];
                if (distance_left > distance_right) && (spatium_start_index + 1 <= ship_demensions.number_spatiums() - 1) {
                    debug!("LoadComponent.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                    let f_x = max_intensity(distance_right);
                    let spatium_function = self.load_intensity_for_spatium(spatium_start_index, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_right);
                    let spatium_function = self.load_intensity_for_spatium(spatium_start_index + 1, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptium are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                } else if (distance_right > distance_left ) && (spatium_start_index - 1 >= 0) {
                    debug!("LoadComponent.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = max_intensity(distance_left);
                    let spatium_function = self.load_intensity_for_spatium(spatium_start_index, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_left);
                    let spatium_function = self.load_intensity_for_spatium(spatium_start_index - 1, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptium are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                } else {
                    debug!("LoadComponent.intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = self.value / ship_demensions.length_spatium();
                    let spatium_function = self.load_intensity_for_spatium(spatium_start_index, ship_demensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptium are under the load {:#?}", load_component_intensity);
                    load_component_intensity
                }

            },
            LoadComponentSpread::WithinManySpatiums => {
                todo!();

            },
            LoadComponentSpread::OutsideLeftmostFrame => {
                todo!();
            }
            LoadComponentSpread::OutsideRightmostFrame => {
                todo!();
            }
        }
    }

    ///
    /// Determine spread of load component.
    /// Returns enum LoadComponentSpread.
    fn spread(&self, ship_demensions: &ShipDimensions) -> LoadComponentSpread {
        let spatium_start_index = self.spatium_start_index(ship_demensions);
        let spatium_end_index = self.spatium_end_index(ship_demensions);
        if spatium_end_index < 0 && spatium_start_index < 0 {
            debug!("LoadComponent.spread | The load component is outside the leftmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("LoadComponent.spread | he lad component: {:#?}", self);
            LoadComponentSpread::OutsideLeftmostFrame
        } else if spatium_end_index > ship_demensions.number_spatiums() - 1 && spatium_start_index > ship_demensions.number_spatiums() - 1 {
            debug!("LoadComponent.spread | The load component is outside the rightmost frame. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("LoadComponent.spread | The load component: {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadComponentSpread::OutsideRightmostFrame
        } else if spatium_end_index - spatium_start_index > 0 {
            debug!("LoadComponent.spread | The load component spreads whithin many spatiums. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("LoadComponent.spread | The load component: {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadComponentSpread::WithinManySpatiums
        } else {
            debug!("LoadComponent.spread | The load component spreads whithin one spatium. start index: {}, end index: {}", spatium_start_index, spatium_end_index);
            debug!("LoadComponent.spread | The load component: {:#?}. ShipDimensions: {:#?}", self, ship_demensions);
            LoadComponentSpread::WithinOneSpatium
        }
    }
}