use crate::filter::Tu8uc1;

#[derive(Debug, Copy, Clone)]
pub struct InciseRangePosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Direction {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct InciseRange {
    pub x: i32,
    pub y: i32,
    pub cols: i32,
    pub rows: i32,
    pub positions: Vec<InciseRangePosition>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub source: Tu8uc1,
    pub is_empty_source: bool,
}

impl PartialEq for InciseRange {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn find_connected_range(
    source: &Tu8uc1,
    sign: &mut Tu8uc1,
    range: &mut InciseRange,
    directions: &Vec<Direction>,
    x: i32,
    y: i32,
    threshold_color_val: u8,
    ox: i32,
    oy: i32,
) {
    for direct in directions {
        let cur_x = direct.x + x + ox;
        let cur_y = direct.y + y + oy;
        let cur_idx: usize = cur_x as usize + cur_y as usize * source.cols as usize;
        if sign.buffer[cur_idx] == 0 && source.buffer[cur_idx as usize] == 255 {
            sign.buffer[cur_idx] = 1;
            range.positions.push(InciseRangePosition {
                x: cur_x - x,
                y: cur_y - y,
            });
            if cur_x < range.min_x {
                range.min_x = cur_x;
            };
            if cur_y < range.min_y {
                range.min_y = cur_y;
            }
            if cur_x > range.max_x {
                range.max_x = cur_x;
            }
            if cur_y > range.max_y {
                range.max_y = cur_y;
            }
            find_connected_range(
                source,
                sign,
                range,
                directions,
                x,
                y,
                threshold_color_val,
                direct.x + ox,
                direct.y + oy,
            );
        }
    }
}

pub fn incise_connected_area(
    source: &mut Tu8uc1,
    need_source: bool,
    through: bool,
) -> Vec<InciseRange> {
    let mut empty = source.empty();
    let sign = &mut empty;
    let mut ranges: Vec<InciseRange> = vec![];
    let mut pixel_idx = 0;
    let directions = if through == false {
        vec![
            Direction { x: 0, y: -1 },
            Direction { x: -1, y: 0 },
            Direction { x: 1, y: 0 },
            Direction { x: 0, y: 1 },
        ]
    } else {
        vec![
            Direction { x: -1, y: -1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: -1 },
            Direction { x: -1, y: 0 },
            Direction { x: 1, y: 0 },
            Direction { x: -1, y: 1 },
            Direction { x: 0, y: 1 },
            Direction { x: 1, y: 1 },
        ]
    };
    let threshold_color_val = if source.fill_value == 255 { 0 } else { 255 };
    for y in 0..source.rows {
        for x in 0..source.cols {
            if sign.buffer[pixel_idx] == 0 && source.buffer[pixel_idx] == threshold_color_val {
                let mut range = InciseRange {
                    x,
                    y,
                    cols: 1,
                    rows: 1,
                    min_x: x,
                    min_y: y,
                    max_x: x,
                    max_y: y,
                    positions: vec![InciseRangePosition { x: 0, y: 0 }],
                    source: Tu8uc1 {
                        buffer: vec![],
                        cols: 0,
                        rows: 0,
                        fill_value: 255,
                    },
                    is_empty_source: true,
                };
                // 找范围
                find_connected_range(
                    source,
                    sign,
                    &mut range,
                    &directions,
                    x,
                    y,
                    threshold_color_val,
                    0,
                    0,
                );
                ranges.push(range);
            }
            sign.buffer[pixel_idx] = 1;
            pixel_idx += 1;
        }
    }

    for i in 0..ranges.len() {
        let r = &mut ranges[i];
        r.cols = r.max_x - r.min_x + 1;
        r.rows = r.max_y - r.min_y + 1;
        let ox = r.x - r.min_x;
        let oy = r.y - r.min_y;
        if need_source {
            let mut buffer: Vec<u8> = vec![0; r.cols as usize * r.rows as usize];
            for ip in 0..r.positions.len() {
                buffer[(r.positions[ip].x + ox + (r.positions[ip].y + oy) * r.cols) as usize] = 255;
            }
            r.source = Tu8uc1 {
                cols: r.cols,
                rows: r.rows,
                buffer,
                fill_value: 0,
            };
            r.is_empty_source = false;
        }
    }
    return ranges;
}

pub fn merge_incise_ranges(ranges: &Vec<InciseRange>, cols: i32, rows: i32) -> Vec<InciseRange> {
    if ranges.is_empty() {
        return vec![];
    }
    let mut range_groups: Vec<Vec<&InciseRange>> = vec![];
    let mut grouped_rang_arr: Vec<&InciseRange> = vec![];
    for i in 0..ranges.len() {
        if !grouped_rang_arr.contains(&&ranges[i]) {
            grouped_rang_arr.push(&&ranges[i]);
            // 判断跟currange_group中的每一个有没有交集
            let pass_idx = range_groups.iter().position(|range_list| {
                let item = range_list.iter().find(|range| {
                    let non_intersect = ranges[i].max_x + cols < range.min_x - cols
                        || ranges[i].min_x - cols > range.max_x + cols
                        || ranges[i].max_y + rows < range.min_y - rows
                        || ranges[i].min_y - rows > range.max_y + rows;
                    return !non_intersect;
                });
                match item {
                    Some(_) => true,
                    None => false,
                }
            });
            match pass_idx {
                Some(idx) => range_groups[idx].push(&&ranges[i]),
                None => range_groups.push(vec![&ranges[i]]),
            }
        }
    }
    let list = range_groups.iter().map(|range_group| {
        let mut r = InciseRange {
            x: range_group[0].x,
            y: range_group[0].y,
            min_x: range_group[0].min_x,
            max_x: range_group[0].max_x,
            min_y: range_group[0].min_y,
            max_y: range_group[0].max_y,
            cols: 0,
            rows: 0,
            // positions: vec![InciseRangePosition { x: 0, y: 0 }],
            source: Tu8uc1 {
                buffer: vec![],
                cols: 0,
                rows: 0,
                fill_value: 255,
            },
            is_empty_source: true,
            positions: range_group[0].positions.clone(),
        };
        for pi in 0..range_group[0].positions.len() {
            r.positions.push(InciseRangePosition {
                x: range_group[0].positions[pi].x,
                y: range_group[0].positions[pi].y,
            });
        }
        for i in 1..range_group.len() {
            if range_group[i].min_x < r.min_x {
                r.min_x = range_group[i].min_x;
            }
            if range_group[i].max_x > r.max_x {
                r.max_x = range_group[i].max_x;
            }
            if range_group[i].min_y < r.min_y {
                r.min_y = range_group[i].min_y;
            }
            if range_group[i].max_y > r.max_y {
                r.max_y = range_group[i].max_y;
            }
            let ox = range_group[i].x - r.x;
            let oy = range_group[i].y - r.y;
            for pi in 0..range_group[i].positions.len() {
                r.positions.push(InciseRangePosition {
                    x: range_group[i].positions[pi].x + ox,
                    y: range_group[i].positions[pi].y + oy,
                });
            }
        }

        r.cols = r.max_x - r.min_x + 1;
        r.rows = r.max_y - r.min_y + 1;

        let mut buffer = vec![0; r.cols as usize * r.rows as usize];
        // Buffer.alloc(.fill(0);
        let ox = r.x - r.min_x;
        let oy = r.y - r.min_y;
        for ip in 0..r.positions.len() {
            buffer[(r.positions[ip].x + ox + (r.positions[ip].y + oy) * r.cols) as usize] = 255;
        }
        r.source = Tu8uc1 {
            cols: r.cols,
            rows: r.rows,
            buffer,
            fill_value: r.source.fill_value,
        };
        r.is_empty_source = true;
        return r;
    });
    list.collect()
}

pub fn incise_scope_aisle(source: &mut Tu8uc1, row: i32, column: i32) -> Vec<InciseRange> {
    let mut empty = source.empty();
    let sign = &mut empty;
    let mut ranges: Vec<InciseRange> = vec![];
    for w in 0..source.cols {
        for h in 0..source.rows {
            if sign.at(w, h) == 0 && source.at(w, h) == 255 {
                let start_x = w;
                let mut start_y = h;
                let mut end_x = w + 1;
                let mut end_y = h + 1;
                let mut unobstructed;
                'loop1: loop {
                    'loop2: loop {
                        unobstructed = true;
                        for x in 0..column {
                            for y in 0..row {
                                if end_x + x >= source.cols || end_y + y >= source.rows {
                                    break;
                                }
                                if end_x >= source.cols - x - 1 {
                                    end_x = source.cols - x - 1;
                                }
                                if end_y >= source.rows - y - 1 {
                                    end_y = source.rows - y - 1;
                                }
                                if source.at(end_x, end_y) == 255 {
                                    end_x += 1;
                                    end_y += 1;
                                    unobstructed = false;
                                }
                            }
                        }
                        if unobstructed {
                            break 'loop2;
                        }
                    }
                    // println!("{}, {}, {}", end_x, end_y, source.at(end_x, end_y));
                    if unobstructed {
                        'for2: for y in 0..row {
                            'for22: for ix in (start_x + 1)..(end_x + column) {
                                if ix >= source.cols || start_y - y - 1 < 0 {
                                    break 'for22;
                                }
                                if source.at(ix, start_y - y - 1) == 255 {
                                    start_y = start_y - 1;
                                    unobstructed = false;
                                    break 'for2;
                                }
                            }
                        }
                    }
                    if unobstructed {
                        for x in 0..column {
                            let mut iy = end_y - 1;
                            while iy >= start_y - 1 {
                                if iy < 0 || iy > source.rows || end_x + x > source.cols {
                                    break;
                                }
                                if source.at(end_x + x, iy) == 255 {
                                    end_x += 1;
                                    unobstructed = false;
                                    break;
                                }
                                iy -= 1;
                            }
                            if unobstructed == false {
                                break;
                            }
                        }
                    }
                    if unobstructed {
                        for y in 0..row {
                            let mut ix = end_x - 1;
                            while ix >= start_x {
                                if ix < 0 || ix > source.cols || end_y + y > source.rows {
                                    break;
                                }
                                if source.at(ix, end_y + y) == 255 {
                                    end_y += 1;
                                    unobstructed = false;
                                    break;
                                }
                                ix -= 1;
                            }
                            if unobstructed == false {
                                break;
                            }
                        }
                    }
                    if unobstructed {
                        break 'loop1;
                    }
                }

                let cols = end_x - start_x;
                let rows = end_y - start_y;

                let mut buffer: Vec<u8> = vec![0; cols as usize * rows as usize];

                let mut buffer_idx = 0;
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        if source.at(x, y) == 255 {
                            sign.set(x, y, 1);
                            buffer[buffer_idx] = 255;
                        }
                        buffer_idx += 1;
                    }
                }

                let range = InciseRange {
                    x: start_x,
                    y: start_y,
                    cols,
                    rows,
                    min_x: start_x,
                    min_y: start_y,
                    max_x: end_x - 1,
                    max_y: end_y - 1,
                    positions: vec![],
                    source: Tu8uc1 {
                        buffer,
                        cols,
                        rows,
                        fill_value: 0,
                    },
                    is_empty_source: true,
                };
                // if range.cols < 9 || range.rows < 9 {
                //     println!("{:?}", range);
                // }
                ranges.push(range);
            }
        }
    }

    ranges
}
