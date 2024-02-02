use plotly::{Scatter, common::{Mode, LineShape, Line, Title, Font}, Layout, layout::{Axis, RangeMode, Legend}, Plot};

use crate::strength::ship::spatium_functions::SpatiumFunctions;

pub struct Visualisation<'a> {
    result: &'a Vec<(f64, f64)>,
    name: String,
    title: String,
    spatium_length: f64,
}



impl<'a> Visualisation<'a> {
    pub fn new(result: &'a Vec<(f64, f64)>, name: String, title: String, spatium_length: f64) -> Self {
        Visualisation { result, name, title, spatium_length, }
    }

    pub fn visualize(&self) {
        let mut x: Vec<f64> = vec![];
        let mut y = vec![];
        for spatium in self.result {
            x.push(spatium.0);
            y.push(spatium.1);
        }

        let trace1 = Scatter::new(x, y)
        .mode(Mode::LinesMarkers)
        .name(&self.name)
        .line(Line::new().shape(LineShape::Linear));
        let mut plot = Plot::new();
        let layout = Layout::new()
            .x_axis(Axis::new().dtick(self.spatium_length))
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new(&self.title))
            .y_axis(Axis::new().range_mode(RangeMode::ToZero));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }
}
