use crate::{incise::InciseRange, libs::TuLib};

#[derive(Debug)]
pub struct OCRText {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub text: String,
}

pub fn ocr_base(lib: &TuLib, ranges: &Vec<InciseRange>, similar: i32) -> Vec<OCRText> {
    let mut ret = vec![];
    ranges.iter().for_each(|range| {
        let mut same_size_lattice_arr = lib
            .data
            .iter()
            .filter(|item| item.cols == range.cols && item.rows == range.rows);
        let lib_text = same_size_lattice_arr.find(|item| {
            let mut num_mismatches = 0;
            let maximum_mismatches = item.num_pixels * ((100 - similar) / 100);
            // println!("{}", maximum_mismatches);
            for i in 0..range.source.buffer.len() {
                if range.source.buffer[i] != item.buffer[i] {
                    num_mismatches += 1;
                    if num_mismatches > maximum_mismatches {
                        return false;
                    }
                }
            }
            return true;
        });
        match lib_text {
            Some(lib_text) => {
                ret.push(OCRText {
                    x: range.x,
                    y: range.y,
                    width: range.cols,
                    height: range.rows,
                    text: String::from(&lib_text.text),
                });
                // println!("{:?}, sum:{}", lib_text.text, ret.len())
            }
            None => (),
        }
    });
    ret
}
