use log::debug;

use crate::{strength::ship::ship_dimensions::ShipDimensions, core::point::Point};

use super::shipload::Shipload;

// pub struct LoadSharing<'shipload> {
//     ship_dimensions: &'shipload ShipDimensions,
//     shipload: &'shipload ShipLoad
// }


// impl<'shipload> LoadSharing<'shipload> {
//     pub fn new(ship_dimensions: &ShipDimensions, shipload: &ShipLoad) -> Self {
//         LoadSharing { ship_dimensions, shipload }
//     }

//     pub fn shared_loads(&self) -> Vec<ShipLoad> {
//         let mut shared_loads: Vec<ShipLoad> = vec![];
//         let x_1 = self.shipload.load_start_coordinate();
//         let x_4 = self.shipload.load_end_coordinate();
//         let spatium_start_index = self.shipload.spatium_start_index(ship_dimensions);
//         let saptium_end_index = self.shipload.spatium_end_index(ship_dimensions);
//         let x_2 = self.ship_dimensions.spatium_end_coordinate(spatium_start_index);
//         let x_3 = self.ship_dimensions.spatium_start_coordinate(saptium_end_index);
//         debug!("x_1 = {}, x_2 = {}, x_3 = {}, x_4 = {}", x_1, x_2, x_3, x_4);
//         if (x_1.abs() - x_2.abs()).abs() > 0.0 {
//             let load = self.shared_shipload(x_1, x_2);
//             shared_loads.push(load);
//         } else if (x_4.abs() - x_3.abs()).abs() > 0.0 {
//             let load = self.shared_shipload(x_3, x_4);
//             shared_loads.push(load);
//         }
//         let mut load_start_coordinate = x_2;
//         let mut load_end_coordinate = x_2 + ship_dimensions.length_spatium();
//         let number_whole_spatiums_under_load = ((x_2.abs() - x_3.abs()).abs() / ship_dimensions.length_spatium()) as i64;
//         for _ in 0..number_whole_spatiums_under_load {
//             let load = self.shared_shipload(load_start_coordinate, load_end_coordinate);
//             shared_loads.push(load);
//             load_start_coordinate += ship_dimensions.length_spatium();
//             load_end_coordinate += ship_dimensions.length_spatium();
//         }
//         shared_loads
//     }

//     fn shared_shipload(&self, load_start_coordinate: f64, load_end_coordinate: f64) -> ShipLoad {
//         let load_length = (load_start_coordinate.abs() - load_end_coordinate.abs()).abs();
//         let longitudinal_center_gravity = load_start_coordinate + (load_length / 2.0);
//         let center_gravity = Point::new(longitudinal_center_gravity, self.shipload.center_gravity.y, self.shipload.center_gravity.z);
//         let load_value = (load_length / self.shipload.length()) * self.shipload.value();
//         ShipLoad::new(load_value, center_gravity, load_length)

//     }
// } 



