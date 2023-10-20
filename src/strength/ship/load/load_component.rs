use log::debug;
use serde::Deserialize;
use crate::{core::point::Point, strength::ship::{ship_dimensions::ShipDimensions, spatium::Spatium}};

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

    fn intensity(&self, spatium_id: i64, ship_demensions: &ShipDimensions, f_x1: f64, f_x2: f64) -> Spatium {
        let spatium_start_coordinate = self.spatium_start_coordinate(spatium_id, ship_demensions);
        let spatium_end_coordinate = self.spatium_end_coordinate(spatium_id, ship_demensions);
        Spatium::new(spatium_id, spatium_start_coordinate, spatium_end_coordinate, f_x1, f_x2)
    }

    ///
    /// Computes load component intensity for a spatium.
    pub fn load_component_intensity(&self, ship_demensions: &ShipDimensions) -> Vec<Spatium> {
        match self.spread(ship_demensions) {
            LoadComponentSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { self.value * (0.5 + (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let min_intensity = |c_min: f64| { self.value * (0.5 - (c_min / ship_demensions.length_spatium())) / ship_demensions.length_spatium() };
                let spatium_index = self.spatium_start_index(ship_demensions);
                let spatium_start_coordinate = self.spatium_start_coordinate(spatium_index, ship_demensions);
                let spatium_end_coordinate = self.spatium_end_coordinate(spatium_index, ship_demensions);
                let c_left = (self.longitudinal_center_gravity() - spatium_start_coordinate).abs();
                let c_right = (self.longitudinal_center_gravity() - spatium_end_coordinate).abs();

                // Ближе к правому шпангоуту теоретической шпации.
                if (c_left > c_right) && (spatium_index + 1 <= ship_demensions.number_spatiums() - 1) {
                    debug!("LoadComponent.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", c_right, c_left);
                    let mut load_component_intensity = vec![Spatium::new(spatium_index, spatium_start_coordinate, spatium_end_coordinate, max_intensity(c_right), max_intensity(c_right))];
                    let spatium_index = spatium_index + 1;
                    let spatium_start_coordinate = self.spatium_start_coordinate(spatium_index, ship_demensions);
                    let spatium_end_coordinate = self.spatium_end_coordinate(spatium_index, ship_demensions);
                    let spatium = Spatium::new(spatium_index, spatium_start_coordinate, spatium_end_coordinate, min_intensity(c_right), min_intensity(c_right));
                    load_component_intensity.push(spatium);
                    debug!("LoadComponent.intensity | Spatiums are under the load: {:#?}", load_component_intensity);
                    return  load_component_intensity;

                // Ближе к левому шпангоуту теоретической шпации
                } else if (c_right > c_left ) && (spatium_index - 1 >= 0) {
                    debug!("LoadComponent.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", c_right, c_left);
                    let mut load_component_intensity = vec![Spatium::new(spatium_index, spatium_start_coordinate, spatium_end_coordinate, max_intensity(c_left), max_intensity(c_left))];
                    let spatium_index = spatium_index - 1;
                    let spatium_start_coordinate = self.spatium_start_coordinate(spatium_index, ship_demensions);
                    let spatium_end_coordinate = self.spatium_end_coordinate(spatium_index, ship_demensions);
                    let spatium = Spatium::new(spatium_index, spatium_start_coordinate, spatium_end_coordinate, min_intensity(c_left), min_intensity(c_left));
                    load_component_intensity.push(spatium);
                    debug!("LoadComponent.intensity | Spatiums are under the load: {:#?}", load_component_intensity);
                    return load_component_intensity;
                } else {
                    debug!("LoadComponent.intensity | Груз расположен на крайней шпации. c_right = {}, c_left = {}", c_right, c_left);
                    let f_x = self.value / ship_demensions.length_spatium();
                    let load_component_intensity = vec![Spatium::new(spatium_index, spatium_start_coordinate, spatium_end_coordinate, f_x, f_x)];
                    debug!("LoadComponent.intensity | Spatiums are under the load: {:#?}", load_component_intensity);
                    return load_component_intensity;
                }

            },
            LoadComponentSpread::WithinManySpatiums => {
                let spatium_start_index = self.spatium_start_index(ship_demensions);
                let spatium_end_index = self.spatium_end_index(ship_demensions);
                let intensity_on_leftmost_spatium = self.intensity(spatium_start_index, ship_demensions, 0.0, 0.0);
                let intesity_on_rightmost_spatium = self.intensity(spatium_end_index, ship_demensions, 0.0, 0.0);
                let mut load_component_intensity = vec![intensity_on_leftmost_spatium, intesity_on_rightmost_spatium];

                for id in spatium_start_index - 1..spatium_end_index {
                    debug!("{}", id);

                }
                load_component_intensity

            },
            LoadComponentSpread::OutsideLeftmostFrame => {
                let mut load_component_intensity = vec![];
                load_component_intensity
            }
            LoadComponentSpread::OutsideRightmostFrame => {
                let mut load_component_intensity = vec![];
                load_component_intensity
            }
        }
    }

    ///
    /// Determine spread of load component.
    /// Returns enum LoadSpread.
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