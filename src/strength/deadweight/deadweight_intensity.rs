use crate::strength::load::shiploads::Shiploads;
use crate::strength::ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

use super::deadweight_intensity_builder::DeadweightIntensityBuilder;

///
/// Интенсивность дедвейта по длине судна.
#[derive(Debug)]
pub struct DeadweightIntensity {
    deadweight_intensity_: SpatiumFunctions,
}

impl DeadweightIntensity {
    ///
    /// Основной конструктор.
    pub fn new(deadweight_intensity_: SpatiumFunctions) -> Self {
        DeadweightIntensity {
            deadweight_intensity_,
        }
    }

    pub fn builder(
        shiploads: &Shiploads,
        ship_dimensions: ShipDimensions,
    ) -> DeadweightIntensityBuilder {
        DeadweightIntensityBuilder::new(shiploads, ship_dimensions)
    }

    ///
    /// Возвращает интенсивность дедвейта по длине судна т/м.
    pub fn deadweight_intensity(&self) -> &SpatiumFunctions {
        &self.deadweight_intensity_
    }
}
