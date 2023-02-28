use crate::incise::InciseRange;
use opencv::{
    core, highgui,
    imgcodecs::{self, IMREAD_COLOR},
    imgproc,
    prelude::*,
    Result,
};

pub fn pixel_from_picture(path: &str) -> Result<Mat> {
    imgcodecs::imread(path, IMREAD_COLOR)
}

pub fn pixel_preview(source: &mut Mat, ranges: &Vec<InciseRange>) -> Result<()> {
    for range in ranges {
        let scaled_face = core::Rect {
            x: range.min_x-1,
            y: range.min_y-1,
            width: range.cols+2,
            height: range.rows+2,
        };
        imgproc::rectangle(
            source,
            scaled_face,
            core::Scalar::new(0f64, 0f64, 255f64, 0f64),
            1,
            1,
            0,
        )?;
    }
    let s = "test".to_string();
    highgui::imshow(&s, source)?;
    highgui::wait_key(0)?;
    Ok(())
}
