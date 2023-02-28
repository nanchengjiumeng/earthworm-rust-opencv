use std::ffi::c_void;

use opencv::{core::Mat_AUTO_STEP, prelude::*};

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Tu8uc1 {
    pub buffer: Vec<u8>,
    pub rows: i32,
    pub cols: i32,
    pub fill_value: u8,
}

impl Tu8uc1 {
    pub fn new(rows: i32, cols: i32, buffer: Vec<u8>, fill_value: u8) -> Tu8uc1 {
        Tu8uc1 {
            buffer,
            rows,
            cols,
            fill_value,
        }
    }
    pub fn to_mat(&mut self) -> Mat {
        unsafe {
            Mat::new_rows_cols_with_data(
                self.rows,
                self.cols,
                u8::typ(),
                self.buffer.as_mut_ptr() as *mut c_void,
                Mat_AUTO_STEP,
            )
            .unwrap()
        }
    }

    pub fn empty(&mut self) -> Tu8uc1 {
        Tu8uc1 {
            buffer: vec![self.fill_value; self.rows as usize * self.cols as usize],
            rows: self.rows,
            cols: self.cols,
            fill_value: self.fill_value,
        }
    }

    pub fn at(&mut self, col: i32, row: i32) -> u8 {
        return self.buffer[(row * self.cols + col) as usize];
    }

    pub fn set(&mut self, col: i32, row: i32, value: u8) {
        self.buffer[(row * self.cols + col) as usize] = value;
    }
}

#[allow(dead_code)]
pub fn filter_binaryzation(data: &Vec<u8>, rows: i32, cols: i32, threshold: &str) -> Tu8uc1 {
    // let data = mat.data_bytes().unwrap().to_vec();
    let data_len = data.len();
    let mut src_data: Vec<u8> = vec![255; data_len / 3];
    let colors: Vec<u8> = threshold
        .split("-")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let mut idx = 0;
    let mut i = 0;
    while i < data_len {
        let pixel_color = data[i] / 3 + data[i + 1] / 3 + data[i + 2] / 3;
        if pixel_color > colors[0] && pixel_color < colors[1] {
            src_data[idx] = 255;
        }
        idx += 1;
        i += 3;
    }
    Tu8uc1::new(rows, cols, src_data, 0)
}

#[allow(dead_code)]
pub fn filter_binaryzation_rgb(
    data: &Vec<u8>,
    rows: i32,
    cols: i32,
    threshold: Vec<RGB>,
) -> Tu8uc1 {
    // let data = mat.data_bytes().unwrap().to_vec();
    let data_len = data.len();
    let mut src_data: Vec<u8> = vec![0; data_len / 3];
    let mut idx = 0;
    let mut i = 0;
    while i < data_len {
        for rgb in &threshold {
            if rgb.b == data[i] && rgb.g == data[i + 1] && rgb.r == data[i + 2] {
                src_data[idx] = 255;
            }
        }

        idx += 1;
        i += 3;
    }
    Tu8uc1::new(rows, cols, src_data, 0)
}
