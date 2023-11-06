use std::fmt::{Display, Formatter};

use crate::{vector::Vector2D, parser::read_signed_int};

fn parse_line(buffer: &[u8], cursor: &mut usize) -> (Vector2D<i32>, Vector2D<i32>) {
    *cursor += 12;
    let sensor_x = read_signed_int(buffer, cursor).unwrap();
    *cursor += 4;
    let sensor_y = read_signed_int(buffer, cursor).unwrap();
    *cursor += 25;
    let beacon_x = read_signed_int(buffer, cursor).unwrap();
    *cursor += 4;
    let beacon_y = read_signed_int(buffer, cursor).unwrap();
    *cursor += 1;
    return (Vector2D{x: sensor_x as i32, y: sensor_y as i32}, Vector2D{x: beacon_x as i32, y: beacon_y as i32})
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    const SCANNED_Y: i32 =  2000000;
    let mut clear_zones = [(0, 0); 30];
    let mut clear_zones_amount = 0;
    let mut beacons_on_row = [0; 10];
    let mut beacons_on_row_amount = 0;

    let mut cursor = 0;
    while cursor < buffer.len() {
        let (sensor, beacon) = parse_line(buffer, &mut cursor);
        if beacon.y == SCANNED_Y {
            let mut already_detected = false;
            for i in 0..beacons_on_row_amount {
                if beacons_on_row[i] == beacon.x {
                    already_detected = true;
                    break;
                }
            }
            if !already_detected {
                beacons_on_row[beacons_on_row_amount] = beacon.x;
                beacons_on_row_amount += 1;
            }
        }


        let manhattan_length = (sensor - beacon).manhattan_length();
        
        if (sensor.y - SCANNED_Y).abs() > manhattan_length {
            continue;
        }

        let vision_width = manhattan_length - (sensor.y - SCANNED_Y).abs();
        let mut clear_zone_start = sensor.x - vision_width;
        let mut clear_zone_end = sensor.x + vision_width;

        let mut writing_offset = 0;
        let mut reading_offset = 0;
        let mut contained = false;

        while reading_offset < clear_zones_amount {

            if clear_zones[reading_offset].0 <= clear_zone_start && clear_zones[reading_offset].1 >= clear_zone_end {
                // New range is already contained
                contained = true;
                break;
            }

            else if clear_zones[reading_offset].0 > clear_zone_end || clear_zones[reading_offset].1 < clear_zone_start {
                // Ranges do not overlap
                clear_zones[writing_offset] = clear_zones[reading_offset];
                writing_offset += 1;
            }

            else if clear_zones[reading_offset].0 >= clear_zone_start && clear_zones[reading_offset].1 <= clear_zone_end {
                // New range is overwriting existing range
            }

            else if clear_zones[reading_offset].0 < clear_zone_start {
                // Ranges partially overlap
                clear_zone_start = clear_zones[reading_offset].0;
            }

            else if clear_zones[reading_offset].1 > clear_zone_end {
                // Ranges partially overlap
                clear_zone_end = clear_zones[reading_offset].1;
            }

            reading_offset += 1;
        }

        if !contained {
            clear_zones[writing_offset] = (clear_zone_start, clear_zone_end);
            clear_zones_amount = writing_offset + 1;
        }
    }

    let mut viewing_units = 0;
    for i in 0..clear_zones_amount {
        viewing_units += clear_zones[i].1 - clear_zones[i].0 + 1;
    }
    viewing_units -= beacons_on_row_amount as i32;
    return viewing_units.to_string()
}

struct ScanArea {
    top: Vector2D<i32>,
    bottom: Vector2D<i32>,
    children: Option<Vec<ScanArea>>,
    fully_scanned: bool,
}

impl Display for ScanArea {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{} Scan Area with points {{ top: {}, bottom: {}}}", if self.fully_scanned {"O"} else {"X"}, self.top, self.bottom))
    }
}

impl ScanArea {
    fn square_with_range(min: i32, max: i32) -> Self {
        let mid_point = (max - min) / 2;
        let half_way_down = min - mid_point;
        let half_way_up = max + mid_point;
        ScanArea {
            top: Vector2D { x: mid_point, y: half_way_down },
            bottom: Vector2D { x: mid_point, y: half_way_up },
            children: Option::None,
            fully_scanned: false
        }
    }

    fn contains_point(&self, point: Vector2D<i32>) -> bool {
        if point.x - point.y > self.top.x - self.top.y {
            // Outside top-right edge
            return false;
        }
        if point.x - point.y < self.bottom.x - self.bottom.y {
            // Outside bottom-left edge
            return false;
        }
        if point.x + point.y < self.top.x + self.top.y {
            // Outside top-left edge
            return false;
        }
        if point.x + point.y > self.bottom.x + self.bottom.y {
            // Outside bottom-right edge
            return false;
        }
        return true;
    }

    fn left(&self) -> Vector2D<i32> {
        let x = (self.top.x + self.top.y + self.bottom.x - self.bottom.y) / 2;
        let y = self.top.x + self.top.y - x;
        return Vector2D { x, y }
    }

    fn right(&self) -> Vector2D<i32> {
        let x = (self.bottom.x + self.bottom.y + self.top.x - self.top.y) / 2;
        let y = self.bottom.x + self.bottom.y - x;
        return Vector2D { x, y }
    }

    fn fully_contains_area(&self, area: &ScanArea) -> bool {
        self.contains_point(area.top) && self.contains_point(area.bottom)
    }

    fn intersection(&self, area: &ScanArea) -> Option<ScanArea> {
        if area.bottom.x - area.bottom.y > self.top.x - self.top.y {
            // Outside top-right edge
            return None;
        }

        if area.top.x - area.top.y < self.bottom.x - self.bottom.y {
            // Outside bottom-left edge
            return None;
        }

        if area.bottom.x + area.bottom.y < self.top.x + self.top.y {
            // Outside top-left edge
            return None;
        }

        if area.top.x + area.top.y > self.bottom.x + self.bottom.y {
            // Outside bottom-right edge
            return None;
        }

        let top_intersection = if self.contains_point(area.top) {
            area.top
        } else if area.contains_point(self.top) {
            self.top
        } else {
            let x = (area.top.x + area.top.y + self.top.x - self.top.y) / 2;
            let y = area.top.x + area.top.y - x;
            let intersection_point = Vector2D { x, y };
            if self.contains_point(intersection_point) {
                intersection_point
            } else {
                let x = (self.top.x + self.top.y + area.top.x - area.top.y) / 2;
                let y = self.top.x + self.top.y - x;
                Vector2D { x, y }
            }
        };

        let bottom_intersection = if self.contains_point(area.bottom) {
            area.bottom
        } else if area.contains_point(self.bottom) {
            self.bottom
        } else {
            let x = (area.bottom.x + area.bottom.y + self.bottom.x - self.bottom.y) / 2;
            let y = area.bottom.x + area.bottom.y - x;
            let intersection_point = Vector2D { x, y };
            if self.contains_point(intersection_point) {
                intersection_point
            } else {
                let x = (self.bottom.x + self.bottom.y + area.bottom.x - area.bottom.y) / 2;
                let y = self.bottom.x + self.bottom.y - x;
                Vector2D { x, y }
            }
        };


        Some(ScanArea {
            top: top_intersection,
            bottom: bottom_intersection, 
            children: None, 
            fully_scanned: self.fully_scanned || area.fully_scanned
        })
    }
    fn in_bounds(&self, min: i32, max: i32) -> bool {
        if self.top.y > max {
            return false;
        }
        if self.bottom.y < min {
            return false;
        }
        if self.left().x > max {
            return false;
        }
        if self.right().x < min {
            return false;
        }
        return true;
    }

    fn clear(&mut self, area: &ScanArea, min: i32, max: i32) -> bool {
        if self.fully_scanned {
            return true;
        }

        if area.fully_contains_area(self) {
            self.fully_scanned = true;
            return true;
        }

        match self.intersection(&area) {
            None => { 
                return false;
            },
            Some(intersection) => match self.children.as_mut() {
                None => {
                    self.children = Some(Vec::with_capacity(5));

                    let mut unscanned_child_in_bounds = false;

                    if self.top.x + self.top.y < intersection.top.x + intersection.top.y {
                        let mut bottom = intersection.left();
                        bottom.x -= 1;
                        bottom.y -= 1;
                        let child_area = ScanArea {
                            top: self.top,
                            bottom,
                            children: None,
                            fully_scanned: false
                        };
                        if child_area.in_bounds(min, max) {
                            unscanned_child_in_bounds = true;
                            self.children.as_mut().unwrap().push(child_area);
                        }
                    }

                    if self.top.y - self.top.x < intersection.top.y - intersection.top.x {
                        let top_x = (intersection.top.x + intersection.top.y + self.top.x - self.top.y) / 2;
                        let top_y = intersection.top.x + intersection.top.y - top_x;

                        let bottom_x = (self.bottom.x + self.bottom.y + intersection.top.x - intersection.top.y) / 2 + 1;
                        let bottom_y = self.bottom.x + self.bottom.y - bottom_x;
                        let child_area = ScanArea {
                            top: Vector2D { x: top_x, y: top_y },
                            bottom: Vector2D { x: bottom_x, y: bottom_y },
                            children: None,
                            fully_scanned: false
                        };
                        if child_area.in_bounds(min, max) {
                            unscanned_child_in_bounds = true;
                            self.children.as_mut().unwrap().push(child_area);
                        }
                    }

                    if self.bottom.x + self.bottom.y > intersection.bottom.x + intersection.bottom.y {
                        let mut top: Vector2D<i32> = intersection.right();
                        top.x += 1;
                        top.y += 1;
                        let child_area = ScanArea {
                            top,
                            bottom: self.bottom,
                            children: None,
                            fully_scanned: false
                        };
                        if child_area.in_bounds(min, max) {
                            unscanned_child_in_bounds = true;
                            self.children.as_mut().unwrap().push(child_area);
                        }
                    }

                    if self.bottom.y - self.bottom.x > intersection.bottom.y - intersection.bottom.x {
                        let bottom_x = (intersection.bottom.x + intersection.bottom.y + self.bottom.x - self.bottom.y) / 2;
                        let bottom_y = intersection.bottom.x + intersection.bottom.y - bottom_x;

                        let top_x = (self.top.x + self.top.y + intersection.bottom.x - intersection.bottom.y) / 2 - 1;
                        let top_y = self.top.x + self.top.y - top_x;
                        let child_area = ScanArea {
                            top: Vector2D { x: top_x, y: top_y },
                            bottom: Vector2D { x: bottom_x, y: bottom_y },
                            children: None,
                            fully_scanned: false
                        };
                        if child_area.in_bounds(min, max) {
                            unscanned_child_in_bounds = true;
                            self.children.as_mut().unwrap().push(child_area);
                        }
                    }


                    self.children.as_mut().unwrap().push(intersection);
                    self.fully_scanned = !unscanned_child_in_bounds;
                    return self.fully_scanned;
                },
                Some(children) => {
                    let mut fully_scanned = true;
                    for child in children {
                        fully_scanned &= child.clear(&intersection, min, max);
                    }
                    self.fully_scanned = fully_scanned;
                    return fully_scanned;
                }
            },
        };
    }

}

fn get_single_remaining_point(area: &ScanArea) -> Option<Vector2D<i32>> {
    match area.children.as_ref() {
        None => Some((area.top + area.bottom) / 2),
        Some(children) => {
            for child in children {
                if !child.fully_scanned {
                    return get_single_remaining_point(child);
                }
            }
            None
        }
    }
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    const RANGE_MIN: i32 = 0;
    const RANGE_MAX: i32 = 4000000;

    let mut scan_area = ScanArea::square_with_range(2 * RANGE_MIN, 2 * RANGE_MAX);

    let mut cursor = 0;
    while cursor < buffer.len() {
        let (sensor, beacon) = parse_line(buffer, &mut cursor);
        let manhattan_length = (sensor - beacon).manhattan_length();
        let cleared_area = ScanArea {
            top: Vector2D { x: sensor.x, y: sensor.y - manhattan_length } * 2,
            bottom: Vector2D { x: sensor.x, y: sensor.y + manhattan_length } * 2,
            children: None,
            fully_scanned: true,
        };

        scan_area.clear(&cleared_area, 2 * RANGE_MIN, 2 * RANGE_MAX);
    }

    let beacon_pos = get_single_remaining_point(&scan_area).unwrap() / 2;
    return (beacon_pos.x as isize * 4000000 + beacon_pos.y as isize).to_string()
}