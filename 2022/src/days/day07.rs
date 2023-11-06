use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use crate::parser::{move_to_next_line, read_unsigned_int};

const FILE_MAX_SIZE: usize = 100000;
const SYSTEM_MAX_SIZE: usize = 70000000;
const SYSTEM_REQUIRED_SIZE: usize = 30000000;

enum ItemType {
    FOLDER(), FILE(usize)
}

struct FolderContents {
    remaining_folders: usize,
    size: usize,
}

impl Eq for FolderContents {}

impl PartialEq<Self> for FolderContents {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl PartialOrd<Self> for FolderContents {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cmp(other))
    }
}

impl Ord for FolderContents {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl FolderContents {
    fn empty() -> Self {
        FolderContents {
            remaining_folders: 0,
            size: 0,
        }
    }
}

fn parse_item_p1(buffer: &[u8], offset: &mut usize) -> Option<ItemType> {
    if buffer[*offset] == '$' as u8 {
        return None
    }

    if buffer[*offset] == 'd' as u8 {
        move_to_next_line(buffer, offset);
        return Some(ItemType::FOLDER());
    }

    let file_size = read_unsigned_int(buffer, offset).unwrap();
    move_to_next_line(buffer, offset);
    return Some(ItemType::FILE(file_size));
}

fn ls_p1(buffer: &[u8], offset: &mut usize) -> FolderContents {
    // ls instruction
    move_to_next_line(buffer, offset);

    let mut contents = FolderContents::empty();

    while *offset < buffer.len() {
        let item = parse_item_p1(buffer, offset);
        if item.is_none() {
            break;
        }

        match item.unwrap() {
            ItemType::FOLDER() => contents.remaining_folders += 1,
            ItemType::FILE(size) => contents.size += size,
        }
    }

    return contents;
}

fn read_folder_p1(buffer: &[u8], offset: &mut usize) -> (usize, usize) {
    let mut folder_contents = ls_p1(buffer, offset);
    let mut answer = 0;
    while folder_contents.remaining_folders > 0 {
        // cd in
        //move_to_next_line(buffer, offset);
        move_to_next_line(buffer, offset);

        let (total, part_of_the_answer) = read_folder_p1(buffer, offset);
        answer += part_of_the_answer;
        folder_contents.size += total;
        folder_contents.remaining_folders -= 1;
        // cd out
        if *offset < buffer.len() {
            move_to_next_line(buffer, offset);
        }
    }

    if folder_contents.size < FILE_MAX_SIZE {
        answer += folder_contents.size;
    }

    return (folder_contents.size, answer);
}


pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut offset = 0;

    // cd /
    move_to_next_line(buffer, &mut offset);
    let (_, answer) = read_folder_p1(buffer, &mut offset);

    return answer.to_string()
}

fn parse_item_p2(buffer: &[u8], offset: &mut usize) -> Option<ItemType> {
    if buffer[*offset] == '$' as u8 {
        return None
    }

    if buffer[*offset] == 'd' as u8 {
        move_to_next_line(buffer, offset);
        return Some(ItemType::FOLDER());
    }

    let file_size = read_unsigned_int(buffer, offset).unwrap();
    move_to_next_line(buffer, offset);
    return Some(ItemType::FILE(file_size));
}

fn ls_p2(buffer: &[u8], offset: &mut usize) -> FolderContents {
    // ls instruction
    move_to_next_line(buffer, offset);

    let mut contents = FolderContents::empty();

    while *offset < buffer.len() {
        let item = parse_item_p2(buffer, offset);
        if item.is_none() {
            break;
        }

        match item.unwrap() {
            ItemType::FOLDER() => contents.remaining_folders += 1,
            ItemType::FILE(size) => contents.size += size,
        }
    }

    return contents;
}

fn read_folder_p2(buffer: &[u8], offset: &mut usize, folders: &mut BinaryHeap<FolderContents>) -> usize {
    let mut folder_contents = ls_p2(buffer, offset);
    while folder_contents.remaining_folders > 0 {
        // cd in
        //move_to_next_line(buffer, offset);
        move_to_next_line(buffer, offset);

        let total = read_folder_p2(buffer, offset, folders);
        folder_contents.size += total;
        folder_contents.remaining_folders -= 1;
        // cd out
        if *offset < buffer.len() {
            move_to_next_line(buffer, offset);
        }
    }

    let folder_size = folder_contents.size;
    folders.push(folder_contents);
    return folder_size;
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut offset = 0;

    let mut folders = BinaryHeap::<FolderContents>::new();

    // cd /
    move_to_next_line(buffer, &mut offset);
    let total_size = read_folder_p2(buffer, &mut offset, &mut folders);
    let remaining_place = SYSTEM_MAX_SIZE - total_size;
    let space_to_clear = SYSTEM_REQUIRED_SIZE - remaining_place;

    let mut to_delete = 0;
    loop {
        let contender = folders.pop().unwrap().size;
        if contender < space_to_clear {
            break;
        }
        to_delete = contender;
    }

    return to_delete.to_string()
}