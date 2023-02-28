use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct TuLibText {
    pub text: String,
    pub cols: i32,
    pub rows: i32,
    pub buffer: Vec<u8>,
    pub num_text_length: i32,
    pub num_pixels: i32,
}

#[derive(Debug)]
pub struct TuLib {
    pub name: String,
    pub data: Vec<TuLibText>,
}

pub fn find_char_bounday(s: &str, index: usize) -> usize {
    if s.len() <= index {
        return index;
    }
    let mut idx = index + 1;
    while !s.is_char_boundary(idx) {
        idx += 1;
    }
    idx - index
}

pub fn lib_load<'a>(path: &str, name: &str) -> TuLib {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    let mut lib = TuLib {
        name: name.to_string(),
        data: vec![],
    };
    file.read_to_string(&mut contents).unwrap();
    contents.split("\r\n").for_each(|item| {
        let mut cols = 0;
        let mut rows = 0;
        let mut rows_start = 0;
        let mut text_end = 0;
        let mut binary_start = 0;
        // for i in 0..item.len() {
        let mut i = 0;
        while i < item.len() {
            let len = find_char_bounday(item, i);
            if i > 0 && &item[i..(i + len)] == "," {
                cols = item[(text_end + len)..i].parse::<i32>().unwrap();
                rows_start = i;
            }
            if &item[i..(i + len)] == "|" {
                if text_end == 0 {
                    text_end = i;
                } else {
                    rows = item[(rows_start + len)..i].parse::<i32>().unwrap();
                    binary_start = i + 1;
                    break;
                }
            }
            i += len;
        }
        let mut buf: Vec<u8> = vec![0; cols as usize * rows as usize];
        let mut idx = 0;
        let mut num_text_length = 0;
        for w in 0..cols {
            for h in 0..rows {
                if &item[(binary_start as usize + idx)..(binary_start as usize + idx + 1)] == "1" {
                    buf[h as usize * cols as usize + w as usize] = 255;
                    num_text_length += 1;
                }
                idx += 1;
            }
        }
        lib.data.push(TuLibText {
            text: String::from(&item[0..text_end]),
            cols,
            rows,
            buffer: buf,
            num_text_length,
            num_pixels: cols * rows,
        });
    });
    // println!("{:?}", lib);

    lib
}
