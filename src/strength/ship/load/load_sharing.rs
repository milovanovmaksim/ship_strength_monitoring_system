use log::debug;

use crate::{strength::ship::ship_dimensions::ShipDimensions, core::point::Point};

use super::shipload::Shipload;

pub struct LoadSharing {
    ship_dimensions: ShipDimensions,
    
}


impl LoadSharing {
    pub fn new(ship_dimensions: ShipDimensions) -> Self {
        LoadSharing { ship_dimensions }
    }

    pub fn shared_loads(&self) {
        
    }

}



