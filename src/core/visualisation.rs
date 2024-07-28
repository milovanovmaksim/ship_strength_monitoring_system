use plotly::{
    common::{Font, Line, LineShape, Mode, Title},
    layout::{Axis, Legend, RangeMode},
    Layout, Plot, Scatter,
};

use crate::strength::strength::Strength;

pub enum DiagrammType {
    LightweightIntensity,
    DeadweightIntensity,
    DisplacementIntensity,
    BuoyancyIntensity,
    TotalShipload,
    ShareForce,
    BendingMoment,
}

impl std::fmt::Display for DiagrammType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagrammType::LightweightIntensity => write!(f, "Lightweight intensity, [т / м]"),
            DiagrammType::DeadweightIntensity => write!(f, "Deadweight intensity, [ т/ м]"),
            DiagrammType::DisplacementIntensity => write!(f, "Displacement intensity, [т / м]"),
            DiagrammType::BuoyancyIntensity => write!(f, "Buoyancy intensity, [т / м]"),
            DiagrammType::TotalShipload => write!(f, "Total shipload, [т / м]"),
            DiagrammType::ShareForce => write!(f, "Share force, [т]"),
            DiagrammType::BendingMoment => write!(f, "Bending moment, [т * м]"),
        }
    }
}

pub struct Visualisation<'a> {
    spatium_length: f64,
    strength: &'a Strength,
}

impl<'a> Visualisation<'a> {
    pub fn new(spatium_length: f64, strength: &'a Strength) -> Self {
        Visualisation {
            spatium_length,
            strength,
        }
    }

    pub fn plot(&self, diagramm_type: DiagrammType) {
        let mut x = vec![];
        let mut y = vec![];
        let results = {
            match diagramm_type {
                DiagrammType::LightweightIntensity => self.strength.lightweight_intensity(),
                DiagrammType::DeadweightIntensity => &self.strength.deadweight_intensity(),
                DiagrammType::DisplacementIntensity => self.strength.displacement_intensity(),
                DiagrammType::TotalShipload => self.strength.total_shipload(),
                DiagrammType::BuoyancyIntensity => self.strength.buoyancy_intensity(),
                DiagrammType::ShareForce => self.strength.share_force(),
                DiagrammType::BendingMoment => self.strength.bending_moment(),
            }
        };
        for spatium in results.as_ref() {
            x.push(spatium.x1());
            x.push(spatium.x2());
            y.push(spatium.f_x1());
            y.push(spatium.f_x2());
        }
        let trace1 = Scatter::new(x, y)
            .mode(Mode::LinesMarkers)
            .line(Line::new().shape(LineShape::Linear));
        let mut plot = Plot::new();
        let layout = Layout::new()
            .x_axis(Axis::new().dtick(self.spatium_length))
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new(&diagramm_type.to_string()))
            .y_axis(Axis::new().range_mode(RangeMode::ToZero));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }
}
