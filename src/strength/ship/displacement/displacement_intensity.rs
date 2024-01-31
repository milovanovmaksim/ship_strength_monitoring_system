use crate::strength::ship::spatium_functions::SpatiumFunctions;


///
/// Displacement intensity is algebraic sum deadweight intensity and lightweight intensity. Measured in [ton/m].
pub struct DisplacementIntensity<'a> {
    deadweight_intensity: &'a SpatiumFunctions,
    lightweight_intnesity: &'a SpatiumFunctions,
}


impl<'a> DisplacementIntensity<'a> {
    pub fn new(deadweight_intensity: &'a SpatiumFunctions, lightweight_intnesity: &'a SpatiumFunctions) -> Self {
        DisplacementIntensity { deadweight_intensity, lightweight_intnesity }
    }

    ///
    /// Computes the deadweight intensity for spatiums.
    pub fn spatium_functions(&self) -> SpatiumFunctions {
        let deadweight_intensity = self.deadweight_intensity.as_ref();
        let lightweight_intnesity = self.lightweight_intnesity.as_ref();
        let spatium_functions = deadweight_intensity.iter().zip(lightweight_intnesity)
            .map(|spatium_functions| {
                spatium_functions.0.add(spatium_functions.1)
            }).collect();
        SpatiumFunctions::new(spatium_functions)

    }
}