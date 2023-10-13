#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeOutput {
    LightweightIntensity,
    DeadweightIntensity,
    DisplacementIntensity,
    BuoyantIntensity,
    TotalLoadIntensity,
    ShearForce,
    BendingMoment,
    Stress,
}