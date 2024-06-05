use crate::strength::ship::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    lightweight::lightweight_intensity::LightweightIntensity, spatium_functions::SpatiumFunctions,
};

pub struct DisplacementIntensity {
    deadweight_intensity: DeadweightIntensity,
    lightweight_intnesity: LightweightIntensity,
}

impl DisplacementIntensity {
    pub fn new(
        deadweight_intensity: DeadweightIntensity,
        lightweight_intnesity: LightweightIntensity,
    ) -> Self {
        DisplacementIntensity {
            deadweight_intensity,
            lightweight_intnesity,
        }
    }

    pub fn spatium_functions(&self) -> SpatiumFunctions {
        let deadweight_intensity = self.deadweight_intensity.deadweight_intensity();
        let lightweight_intnesity = self.lightweight_intnesity.lightweight_intensity();
        let spatium_functions = deadweight_intensity
            .into_iter()
            .zip(lightweight_intnesity)
            .map(|spatium_functions| spatium_functions.0.add(spatium_functions.1))
            .collect();
        SpatiumFunctions::new(spatium_functions)
    }
}
