pub mod filter;
pub mod image;
pub mod incise;
pub mod libs;
pub mod ocr;

pub use opencv::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    use filter::RGB;
    use incise::{incise_connected_area, incise_scope_aisle, merge_incise_ranges};

    use crate::{
        filter::filter_binaryzation_rgb,
        image::pixel_from_picture,
        // image::{pixel_from_picture, pixel_preview},
        libs::lib_load,
        ocr::ocr_base,
    };
    #[test]
    fn test_preview_connect_area() {
        let img = pixel_from_picture("tests/1.bmp").unwrap();
        let data = img.data_bytes().unwrap().to_vec();
        let now = Instant::now();
        let lib1 = lib_load("tests/song9UTF8.lib", "lib1");
        let mut filtered_data = filter_binaryzation_rgb(
            &data,
            img.rows(),
            img.cols(),
            vec![RGB {
                r: 255,
                g: 255,
                b: 255,
            }],
        );
        let elapsed_time = now.elapsed();
        let ranges = incise_connected_area(&mut filtered_data, false, true);
        let ranges2 = merge_incise_ranges(&ranges, 1, 1);
        let ocr_ret = ocr_base(&lib1, &ranges2, 100);
        println!("result: {:?}", ocr_ret);
        println!(
            "执行时间(ns): {}, 文字个数: {}",
            elapsed_time.as_micros(),
            ocr_ret.len()
        );
        // pixel_preview(&mut img, &ranges2).unwrap();
    }

    #[test]
    fn test_preview_scope_aisle() {
        let img = pixel_from_picture("tests/52.bmp").unwrap();
        let data = img.data_bytes().unwrap().to_vec();
        let now = Instant::now();
        let lib1 = lib_load("tests/shadowUTF8.lib", "lib1");
        let mut filtered_data = filter::filter_binaryzation_rgb(
            &data,
            img.rows(),
            img.cols(),
            vec![RGB {
                r: 25,
                g: 25,
                b: 25,
            }],
        );
        // let mut m = filtered_data.to_mat();
        // pixel_preview(&mut m, &vec![]).unwrap();
        let elapsed_time = now.elapsed();
        let ranges = incise_scope_aisle(&mut filtered_data, 2, 2);

        let ocr_ret = ocr_base(&lib1, &ranges, 100);
        println!("result: {:?}", ocr_ret);
        println!(
            "执行时间(ns): {}, 文字个数: {}",
            elapsed_time.as_micros(),
            ranges.len()
        );

        // for i in 0..ranges.len() {
        // let mut m = ranges[i].source.to_mat();
        // println!("{:?}", ranges[i].source.buffer);
        // println!("{:?}", lib1.data[20]);
        // pixel_preview(&mut m, &vec![]).unwrap();
        // }
        // pixel_preview(&mut img, &ranges).unwrap();
    }
}
