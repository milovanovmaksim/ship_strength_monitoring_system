use plotly::{Scatter, common::{Mode, LineShape, Line, Title, Font}, Layout, layout::{Axis, RangeMode, Legend}, Plot};

use crate::strength::ship::{spatium_functions::SpatiumFunctions, ship_dimensions::ShipDimensions};

pub struct Visualisation {
    spatiumfunctions: SpatiumFunctions,
    name: String,
    title: String,
    spatium_length: f64,
}



impl Visualisation {
    pub fn new(spatiumfunctions: SpatiumFunctions, name: String, title: String, spatium_length: f64) -> Self {
        Visualisation { spatiumfunctions, name, title, spatium_length, } 
    }

    pub fn visualize(&self) {
        let mut x: Vec<f64> = vec![];
        let mut y = vec![];
        for spatium in self.spatiumfunctions.functions() {
            x.push(spatium.x1());
            y.push(spatium.f_x1());
            x.push(spatium.x2());
            y.push(spatium.f_x2());
        }
        let trace1 = Scatter::new(x, y)
        .mode(Mode::LinesMarkers)
        .name(&self.name)
        .line(Line::new().shape(LineShape::Linear));
        let mut plot = Plot::new();
        let layout = Layout::new()
            .x_axis(Axis::new().dtick(self.spatium_length))
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new(&self.title));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }
}
