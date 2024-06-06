use crate::strength::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight};

pub(crate) struct DisplacementTonnage {
    lightweight: Lightweight,
    deadweight: Deadweight
}