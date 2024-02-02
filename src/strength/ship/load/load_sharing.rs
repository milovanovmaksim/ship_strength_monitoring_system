use crate::strength::ship::{ship_dimensions::ShipDimensions, load::shipload::Shipload};



pub struct LoadSharing<'a> {
    ship_dimensions: &'a ShipDimensions,
    shipload: &'a Shipload
}


impl<'a> LoadSharing<'a> {
    pub fn new(ship_dimensions: &'a ShipDimensions, shipload: &'a Shipload) -> Self {
        LoadSharing { ship_dimensions, shipload }
    }

    ///
    /// Share the shipload by spatiums.
    pub fn shared_loads(&self) -> Vec<Shipload> {
        let mut shared_shiploads = vec![];
        let mut load_start_coordinate = self.shipload.load_start_coordinate();
        let load_end_coordinate = self.shipload.load_end_coordinate();
        let mut current_coordinate = self.ship_dimensions.coordinate_aft();
        let spatium_length = self.ship_dimensions.length_spatium();
        while current_coordinate < load_end_coordinate {
            if current_coordinate > load_start_coordinate {
                let shipload = self.shipload.shared_shipload(load_start_coordinate, current_coordinate);
                shared_shiploads.push(shipload);
                load_start_coordinate = current_coordinate;
            }
            current_coordinate += spatium_length;
        }
        let shipload = self.shipload.shared_shipload(load_start_coordinate, load_end_coordinate);
        shared_shiploads.push(shipload);
        shared_shiploads
    }
}


