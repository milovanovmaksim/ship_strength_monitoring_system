use plotly::{
    common::{Font, Line, LineShape, Mode, Title},
    layout::{Axis, Legend, RangeMode},
    Layout, Plot, Scatter,
};

use crate::strength::{ship::spatium_functions::SpatiumFunctions, strength::Strength};

pub enum DiagrammType {
    LightweightIntensity,
    DeadweightIntensity,
    DisplacementIntensity,
    BuoyancyIntensity,
    TotalShipload,
    ShareForce,
    ShareForceWithCorrection,
    BendingMoment,
    BendingMomentWithCorrection,
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
            DiagrammType::ShareForceWithCorrection => write!(f, "Share force with correction, [т]"),
            DiagrammType::BendingMoment => write!(f, "Bending moment, [т * м]"),
            DiagrammType::BendingMomentWithCorrection => {
                write!(f, "Bending moment with correction, [т * м]")
            }
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

    pub fn show(&self, diagramm_type: DiagrammType) {
        match diagramm_type {
            DiagrammType::LightweightIntensity => {
                self.plot(
                    self.strength.lightweight_intensity(),
                    diagramm_type.to_string(),
                )
                .show();
            }
            DiagrammType::DeadweightIntensity => {
                self.plot(
                    &self.strength.deadweight_intensity(),
                    diagramm_type.to_string(),
                )
                .show();
            }
            DiagrammType::DisplacementIntensity => {
                self.plot(
                    self.strength.displacement_intensity(),
                    diagramm_type.to_string(),
                )
                .show();
            }
            DiagrammType::TotalShipload => self
                .plot(self.strength.total_shipload(), diagramm_type.to_string())
                .show(),
            DiagrammType::BuoyancyIntensity => self
                .plot(
                    self.strength.buoyancy_intensity(),
                    diagramm_type.to_string(),
                )
                .show(),
            DiagrammType::ShareForce => self
                .plot(self.strength.share_force(), diagramm_type.to_string())
                .show(),
            DiagrammType::ShareForceWithCorrection => {
                match self.strength.share_force_with_correction() {
                    Some(share_force) => {
                        self.plot(share_force, diagramm_type.to_string()).show();
                    }
                    _ => (),
                }
            }
            DiagrammType::BendingMoment => {
                self.plot(self.strength.bending_moment(), diagramm_type.to_string())
                    .show();
            }
            DiagrammType::BendingMomentWithCorrection => {
                match self.strength.share_force_with_correction() {
                    Some(bending_moment) => {
                        self.plot(bending_moment, diagramm_type.to_string()).show();
                    }
                    _ => (),
                }
            }
        };
    }

    pub fn plot(&self, s_fs: &SpatiumFunctions, diagramm_type: String) -> Plot {
        let mut x = vec![];
        let mut y = vec![];
        for spatium in s_fs.as_ref() {
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
            .title(Title::new(&diagramm_type))
            .y_axis(Axis::new().range_mode(RangeMode::ToZero));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot
    }
}
