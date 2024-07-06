use crate::core::round::Round;
use crate::{core::point::Point, strength::ship::ship_dimensions::ShipDimensions};
use serde::Deserialize;
use std::fmt::Debug;

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
        Shipload {
            value,
            center_gravity,
            length,
        }
    }

    pub(crate) fn from_id(id: u64, ship_dimensions: &ShipDimensions, value: f64) -> Shipload {
        let spatium_start_coordinate = ship_dimensions.spatium_start_coordinate(id).my_round(2);
        let length_spatium = ship_dimensions.length_spatium();
        let x = spatium_start_coordinate + length_spatium / 2.0;
        let center_gravity = Point::new(x, 0.0, 0.0);
        Shipload::new(value, center_gravity, length_spatium)
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
        let spatium_start_index =
            ship_dimensions.spatium_index_by_coordinate(self.longitudinal_center_gravity());
        let spatium_start_coordinate =
            ship_dimensions.spatium_start_coordinate(spatium_start_index);
        let spatium_end_coordinate = ship_dimensions.spatium_end_coordinate(spatium_start_index);
        let distance_left = (self.longitudinal_center_gravity() - spatium_start_coordinate)
            .abs()
            .my_round(2);
        let distance_right = (self.longitudinal_center_gravity() - spatium_end_coordinate)
            .abs()
            .my_round(2);
        (distance_left, distance_right)
    }

    ///
    /// Share shipload by coordinates.
    /// Params:
    /// load_start_coordinate - coordinate of the start of the new shipload.
    /// load_end_coordinate - coordinate of the end of the new shipload.
    /// Return: Shipload.
    fn shared_shipload(&self, load_start_coordinate: f64, load_end_coordinate: f64) -> Shipload {
        let load_length = (load_end_coordinate - load_start_coordinate).abs();
        let load_value = (load_length / self.length) * self.value;
        let x = (load_start_coordinate + (load_length / 2.0));
        let center_gravity = Point::new(x, self.center_gravity.y, self.center_gravity.z);
        Shipload::new(load_value, center_gravity, load_length)
    }

    pub fn moment(&self) -> f64 {
        self.value * self.longitudinal_center_gravity()
    }

    ///
    /// Share the shipload by spatiums.
    pub fn shared_shiploads(&self, ship_dimensions: &ShipDimensions) -> Vec<Shipload> {
        let mut shared_shiploads = vec![];
        let mut load_start_coordinate = self.load_start_coordinate();
        let spatium_shipload_start_index =
            ship_dimensions.spatium_index_by_coordinate(load_start_coordinate);
        let load_end_coordinate = self.load_end_coordinate();
        let mut current_coordinate =
            ship_dimensions.spatium_start_coordinate(spatium_shipload_start_index);
        let spatium_length = ship_dimensions.length_spatium();
        while current_coordinate < load_end_coordinate {
            if current_coordinate > load_start_coordinate {
                let shipload = self.shared_shipload(load_start_coordinate, current_coordinate);
                shared_shiploads.push(shipload);
                load_start_coordinate = current_coordinate;
            }
            current_coordinate += spatium_length;
        }
        let shipload = self.shared_shipload(load_start_coordinate, load_end_coordinate);
        shared_shiploads.push(shipload);
        shared_shiploads
    }
}

impl PartialEq for Shipload {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}
