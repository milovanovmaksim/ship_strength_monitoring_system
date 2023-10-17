use plotly::{Scatter, common::{Mode, Line, LineShape, Font, Title}, Plot, Layout, layout::{Legend, Axis, RangeMode}};

use crate::strength::ship::spatium::Spatium;

use super::type_output::TypeOutput;


pub struct Output {
    spatiums: Vec<Spatium>,
    type_output: TypeOutput,
}


impl Output {
    pub fn new(spatiums: Vec<Spatium>, type_output: TypeOutput) -> Self {
        Output { spatiums, type_output }

    }

    pub fn draw(&self) {
        let mut x: Vec<f64> = vec![];
        let mut y = vec![];
        for spatium in &self.spatiums {
            x.push(spatium.x1());
            y.push(spatium.f_x1());
            x.push(spatium.x2());
            y.push(spatium.f_x2());
        }
        let trace1 = Scatter::new(x, y)
        .mode(Mode::LinesMarkers)
        .name("name")
        .line(Line::new().shape(LineShape::Linear));
        let mut plot = Plot::new();
        let layout = Layout::new()
            .y_axis(Axis::new().range_mode(RangeMode::ToZero).dtick(2.0))
            .x_axis(Axis::new().dtick(5.0))
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new("Lightweight intensity"));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }

    pub fn len(&self) -> usize {
        self.spatiums.len()
    }

    pub fn integral(&self) -> f64 {
        // TODO: Напиcать класс, который будет интегрировать шпации.
        let mut integral = 0.0;
        for spatium in &self.spatiums {
            integral += spatium.square();
        }
        integral
    }

    pub fn type_output(&self) -> TypeOutput {
        self.type_output
    }


}

