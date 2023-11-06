use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use crate::parser::{move_to_next_line, read_unsigned_int};


pub(crate) fn compare_elements(left_buff: &[u8],
                               left_offset: &mut usize,
                               right_buff: &[u8],
                               right_offset: &mut usize) -> Option<bool> {
    let left_array = left_buff[*left_offset] == '[' as u8;
    let right_array = right_buff[*right_offset] == '[' as u8;

    if left_array && right_array {
        loop {
            *left_offset += 1;
            *right_offset += 1;

            if left_buff[*left_offset] == ']' as u8 || right_buff[*right_offset] == ']' as u8 {
                break;
            }

            let comparison = compare_elements(left_buff, left_offset, right_buff, right_offset);
            if comparison.is_some() {
                return comparison;
            }

            if left_buff[*left_offset] == ']' as u8 || right_buff[*right_offset] == ']' as u8 {
                break;
            }
        }

        if left_buff[*left_offset] == ']' as u8 && right_buff[*right_offset] == ']' as u8 {
            *right_offset += 1;
            *left_offset += 1;
            return None;
        }

        return Some(left_buff[*left_offset] == ']' as u8);

    } else if left_array {
        *left_offset += 1;
        let comparison = compare_elements(left_buff, left_offset, right_buff, right_offset);
        if comparison.is_some() {
            return comparison;
        }
        if left_buff[*left_offset] == ']' as u8 {
            *left_offset += 1;
            return None;
        }
        return Some(false);
    } else if right_array {
        *right_offset += 1;
        let comparison = compare_elements(left_buff, left_offset, right_buff, right_offset);
        if comparison.is_some() {
            return comparison;
        }
        if right_buff[*right_offset] == ']' as u8 {
            *right_offset += 1;
            return None;
        }
        return Some(true);
    }

    if left_buff[*left_offset] == ']' as u8 {
        return Some(true)
    }

    if right_buff[*right_offset] == ']' as u8 {
        return Some(false)
    }

    let left_value = read_unsigned_int(left_buff, left_offset).unwrap();
    let right_value = read_unsigned_int(right_buff, right_offset).unwrap();

    if left_value != right_value {
        return Some(left_value < right_value);
    }

    return None;
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut offset = 0;
    let mut index = 1;
    let mut valid_amount = 0;
    loop {
        let mut left_offset = offset;
        move_to_next_line(buffer, &mut offset);
        let mut right_offset = offset;
        let valid = compare_elements(buffer,
                                     &mut left_offset,
                                     buffer,
                                     &mut right_offset).unwrap();
        if valid {
            valid_amount += index;
        }

        move_to_next_line(buffer, &mut offset);
        if offset >= buffer.len() {
            break;
        }

        move_to_next_line(buffer, &mut offset);
        index += 1;
    }
    return valid_amount.to_string()
}

struct Packet {
    buffer:  *const [u8],
    offset: usize,
}

impl Eq for Packet  {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe {
            let mut self_offset = self.offset;
            let mut other_offset = other.offset;
            return match compare_elements(self.buffer.as_ref().unwrap(),
                                          &mut self_offset,
                                          other.buffer.as_ref().unwrap(),
                                          &mut other_offset) {
                None => Ordering::Equal,
                Some(order) => match order {
                    true => Ordering::Greater,
                    false => Ordering::Less,
                }
            };
        }
    }
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let additional_two = "[[2]]\n".as_bytes();
    let additional_six = "[[6]]\n".as_bytes();

    let mut index_2 = 1;
    let mut index_6 = 2;

    let mut offset = 0;
    loop {
        let mut o1 = offset;
        let mut o2 = offset;
        if compare_elements(buffer, &mut o1, additional_two, &mut 0).unwrap() {
            index_6 += 1;
            index_2 += 1;
        } else if compare_elements(buffer, &mut o2, additional_six, &mut 0).unwrap() {
            index_6 += 1;
        }
        move_to_next_line(buffer, &mut offset);
        o1 = offset;
        o2 = offset;
        if compare_elements(buffer, &mut o1, additional_two, &mut 0).unwrap() {
            index_6 += 1;
            index_2 += 1;
        } else if compare_elements(buffer, &mut o2, additional_six, &mut 0).unwrap() {
            index_6 += 1;
        }
        move_to_next_line(buffer, &mut offset);

        if offset >= buffer.len() {
            break;
        }

        move_to_next_line(buffer, &mut offset);
    }


    return (index_2 * index_6).to_string()
}
pub(crate) fn part2_old(buffer: &[u8]) -> String {
    let additional_two = "[[2]]\n".as_bytes();
    let additional_six = "[[6]]\n".as_bytes();

    let mut ordered_packets = BinaryHeap::new();
    ordered_packets.push(Packet {
        buffer: additional_two as *const [u8],
        offset: 0,
    });
    ordered_packets.push(Packet {
        buffer: additional_six as *const [u8],
        offset: 0,
    });

    let mut offset = 0;
    loop {
        ordered_packets.push(Packet { buffer: buffer as *const [u8], offset });
        move_to_next_line(buffer, &mut offset);
        ordered_packets.push(Packet { buffer: buffer as *const [u8], offset });
        move_to_next_line(buffer, &mut offset);

        if offset >= buffer.len() {
            break;
        }

        move_to_next_line(buffer, &mut offset);
    }

    let mut index = 1;
    let mut product = 1;

    while !ordered_packets.is_empty() {
        let p = ordered_packets.pop().unwrap();
        if p.buffer == additional_two  as *const [u8] {
            product *= index;
        } else if p.buffer == additional_six  as *const [u8] {
            product *= index;
            break;
        }
        index += 1;
    }

    return product.to_string()
}