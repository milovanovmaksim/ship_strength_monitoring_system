use crate::core::linear_interpolation::LinearInterpolation;

use super::frame::Frame;

pub(crate) struct InterpolatedFrame<'a> {
    frame_0: &'a Frame,
    frame_1: &'a Frame,
}



impl<'a> InterpolatedFrame<'a> {
    pub fn new(frame_0: &'a Frame, frame_1: &'a Frame) -> Self {
        InterpolatedFrame { frame_0, frame_1 }
    }


    pub fn interpolated_frame(&self, abscissa: f64) -> Frame {
        let mut volumes = vec![];
        let mut areas = vec![];
        let mut massas = vec![];
        let drafts = self.frame_0.drafts().clone();
        for i in 0..self.frame_0.drafts().len() {
            let abscissa_0 = self.frame_0.abscissa();
            let abscissa_1 = self.frame_1.abscissa();

            let area_0 = *self.frame_0.areas().get(i).unwrap();
            let area_1 = *self.frame_1.areas().get(i).unwrap();
            let linear_interpolation = LinearInterpolation::new(area_0, area_1, abscissa_0, abscissa_1);
            let area = linear_interpolation.interpolated_value(abscissa).unwrap();
            areas.push(area);

            let volume_0 = *self.frame_0.volumes().get(i).unwrap();
            let volume_1 = *self.frame_1.volumes().get(i).unwrap();
            let linear_interpolation = LinearInterpolation::new(volume_0, volume_1, abscissa_0, abscissa_1);
            let volume = linear_interpolation.interpolated_value(abscissa).unwrap();
            volumes.push(volume);


            let massa_0 = *self.frame_0.masses().get(i).unwrap();
            let massa_1 = *self.frame_1.masses().get(i).unwrap();
            let linear_interpolation = LinearInterpolation::new(massa_0, massa_1, abscissa_0, abscissa_1);
            let massa = linear_interpolation.interpolated_value(abscissa).unwrap();
            massas.push(massa);
        }
        Frame::new(1, drafts, areas, volumes, massas, abscissa)
    }

}