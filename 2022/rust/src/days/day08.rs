use std::alloc::{alloc, Layout};
use std::marker::PhantomData;
use std::{alloc, mem};
use std::ptr::NonNull;

const GRID_SIZE: usize = 99;
const TREE_COUNT: usize = GRID_SIZE * GRID_SIZE;
const SECTION_SIZE: usize = 64;
const AMOUNT_SECTIONS: usize = (TREE_COUNT + GRID_SIZE) / SECTION_SIZE + 1;

enum Direction {
    UP, DOWN, LEFT, RIGHT
}

struct Visibility {
    visible: [u64; AMOUNT_SECTIONS],
    amount_visible: usize,
}

impl Visibility {
    fn new() -> Self {
        Visibility {
            amount_visible: 0,
            visible: [0; AMOUNT_SECTIONS],
        }
    }

    fn set_visible(&mut self, offset: usize) {
        let section = offset / SECTION_SIZE;
        let section_offset = offset % SECTION_SIZE;
        let visibility_mask = 1 << section_offset;
        if self.visible[section] & visibility_mask == 0 {
            self.amount_visible += 1;
            self.visible[section] |= visibility_mask;
        }
    }
}

fn scan_row(buffer: &[u8], forest_size: usize, direction: Direction, index: usize, visibility: &mut Visibility, stop_at: u8) -> u8 {
    let mut offset = match direction {
        Direction::UP => (forest_size + 1) * (forest_size - 1) + index,
        Direction::DOWN => index,
        Direction::LEFT => (forest_size + 1) * (index + 1) - 2,
        Direction::RIGHT => (forest_size + 1) * index,
    };

    let mut height = buffer[offset];
    visibility.set_visible(offset);

    for _ in 0..forest_size-1 {
        match direction {
            Direction::UP => offset -= forest_size + 1,
            Direction::DOWN => offset += forest_size + 1,
            Direction::LEFT => offset -= 1,
            Direction::RIGHT => offset += 1,
        }

        if buffer[offset] > height {
            visibility.set_visible(offset);
            height = buffer[offset];
        }

        if height == stop_at {
            return height
        }
    }

    return height;
}


pub(crate) fn part1(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut visibility = Visibility::new();

    for index in 0..GRID_SIZE {
        let stop_at = scan_row(buffer, GRID_SIZE, Direction::LEFT, index, &mut visibility, '9' as u8);
        scan_row(buffer, GRID_SIZE, Direction::RIGHT, index, &mut visibility, stop_at);
        let stop_at = scan_row(buffer, GRID_SIZE, Direction::UP, index, &mut visibility, '9' as u8);
        scan_row(buffer, GRID_SIZE, Direction::DOWN, index, &mut visibility, stop_at);
    }

    return visibility.amount_visible.to_string()
}


#[derive(Copy, Clone)]
struct Peak {
    position: usize,
    height: u8,
}

pub trait StripSurveyor {
    fn move_forward(&mut self, height: u8);
}

#[derive(Copy, Clone)]
struct StackStripSurveyor {
    ascending: [Peak; 10],
    descending: [Peak; 10],
    amount_ascending: usize,
    amount_descending: usize,
    offset: usize
}

impl StripSurveyor for StackStripSurveyor {
    fn move_forward(&mut self, height: u8) {
        if self.offset == 0 || height > self.ascending[self.amount_ascending - 1].height {
            self.ascending[self.amount_ascending] = Peak { position: self.offset, height };
            self.amount_ascending += 1;
            self.descending[0] = Peak { position: self.offset, height };
            self.amount_descending = 1;
        } else {
            let mut backward_offset = self.amount_descending;
            while backward_offset > 0 && self.descending[backward_offset-1].height <= height {
                backward_offset -= 1;
            }
            self.descending[backward_offset] = Peak { position: self.offset, height };
            self.amount_descending = backward_offset + 1;
        }

        self.offset += 1;
    }
}

pub trait ForestSurveyor {
    fn move_row_forward(&mut self, index: usize, height: u8);
    fn move_col_forward(&mut self, index: usize, height: u8);
}

#[derive(Copy, Clone)]
struct StackForestSurveyor {
    rows: [StackStripSurveyor; GRID_SIZE],
    columns: [StackStripSurveyor; GRID_SIZE],
}

impl ForestSurveyor for StackForestSurveyor {
    fn move_row_forward(&mut self, index: usize, height: u8) {
        self.rows[index].move_forward(height);
    }
    fn move_col_forward(&mut self, index: usize, height: u8) {
        self.columns[index].move_forward(height);
    }
}

impl StackForestSurveyor {
    fn new() -> Self {
        let peak = Peak { height: 0, position: 0 };
        let strip_surveyor = StackStripSurveyor {
            ascending: [peak; 10],
            descending: [peak; 10],
            amount_ascending: 0,
            amount_descending: 0,
            offset: 0,
        };
        return StackForestSurveyor {
            rows: [strip_surveyor; GRID_SIZE],
            columns: [strip_surveyor; GRID_SIZE],
        }
    }
}



#[derive(Copy, Clone)]
struct MemStripSurveyor {
    amount_ascending: isize,
    amount_descending: isize,
    offset: usize,
    ptr: *mut Peak,
}

impl StripSurveyor for MemStripSurveyor {
    fn move_forward(&mut self, height: u8) {
        unsafe {
            if self.offset == 0 || height > self.ptr.offset(self.amount_ascending - 1).read().height {
                self.ptr.offset(self.amount_ascending).write(Peak { position: self.offset, height });
                self.amount_ascending += 1;
                self.ptr.offset(10).write(Peak { position: self.offset, height });
                self.amount_descending = 1;
            } else {
                let mut backward_offset = self.amount_descending;
                while backward_offset > 0 && self.ptr.offset(backward_offset + 9).read().height >= height {
                    backward_offset -= 1;
                }
                self.ptr.offset(10 + backward_offset).write(Peak { position: self.offset, height });
                self.amount_descending = backward_offset;
            }

            self.offset += 1;
        }
    }
}

struct MemForestSurveyor {
    rows: [MemStripSurveyor; GRID_SIZE],
    columns: [MemStripSurveyor; GRID_SIZE],
    ptr: NonNull<Peak>,
    cap: usize,
    _marker: PhantomData<Peak>,
}

impl ForestSurveyor for MemForestSurveyor {
    fn move_row_forward(&mut self, index: usize, height: u8) {
        self.rows[index].move_forward(height);
    }
    fn move_col_forward(&mut self, index: usize, height: u8) {
        self.columns[index].move_forward(height);
    }
}

impl MemForestSurveyor {
    fn new() -> Self {
        unsafe {
            let cap = GRID_SIZE * 2 * 10 * 2;
            let layout = Layout::array::<Peak>(cap).unwrap();
            assert!(layout.size() <= isize::MAX as usize, "Allocation too large");
            let ptr = NonNull::new(unsafe { alloc(layout) } as *mut Peak).unwrap();

            let mem_strip_surveyor = MemStripSurveyor {
                amount_ascending: 0,
                amount_descending: 0,
                offset: 0,
                ptr: ptr.as_ptr(),
            };
            let mut surveyor = MemForestSurveyor {
                ptr,
                cap,
                _marker: PhantomData,
                rows: [mem_strip_surveyor; GRID_SIZE],
                columns: [mem_strip_surveyor; GRID_SIZE],
            };

            for i in 0..GRID_SIZE {
                surveyor.columns[i].ptr = ptr.as_ptr().offset((i * 2 * 10) as isize);
                surveyor.rows[i].ptr = ptr.as_ptr().offset(((i + GRID_SIZE) * 2 * 10) as isize);
            }

            return surveyor;
        }
    }
}

impl Drop for MemForestSurveyor {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<Peak>(self.cap).unwrap(),
            );
        }
    }
}

fn generate_stack_forest_surveyor() -> Box<dyn ForestSurveyor> {
    Box::new(StackForestSurveyor::new())
}

fn generate_memory_forest_surveyor() -> Box<dyn ForestSurveyor> {
    Box::new(MemForestSurveyor::new())
}

fn part1_unfinished_forest_surveyor(buffer: &[u8], calculator_generator: impl Fn() -> Box<dyn ForestSurveyor>) -> String {
    let mut offset = 0;
    let mut calculators = calculator_generator();

    for row in 0..GRID_SIZE {
        for column in 0..GRID_SIZE {
            let height = buffer[offset];
            calculators.move_col_forward(column, height);
            calculators.move_row_forward(row, height);
        }
        offset += 1;
    }

    return "".to_string()
}

/**
 * Long to init
 * Long execution
 */
pub(crate) fn part1_unfinished_stack_forest_surveyor(input: &String) -> String {
    return part1_unfinished_forest_surveyor(input.as_bytes(), generate_stack_forest_surveyor);
}

/**
 * Slow to init
 * Even longer execution
 */
pub(crate) fn part1_unfinished_mem_forest_surveyor(input: &String) -> String {
    return part1_unfinished_forest_surveyor(input.as_bytes(), generate_memory_forest_surveyor);
}



#[derive(Copy, Clone)]
struct BackwardSurveyor {
    highest: u8,
    descending: [Peak; 10],
    amount_descending: usize,
}

impl BackwardSurveyor {
    fn move_forward(&mut self, height: u8, offset: usize) {
        if height > self.highest {
            self.highest = height;
            self.descending[0] = Peak { position: offset, height };
            self.amount_descending = 1;
        } else {
            let previous_first_seen = self.descending[self.amount_descending-1];
            if previous_first_seen.height == height {
                return;
            }

            let mut write_at = self.amount_descending;
            if previous_first_seen.height < height {
                while write_at > 0 && self.descending[write_at - 1].height <= height {
                    write_at -= 1;
                }
            }
            self.descending[write_at] = Peak { position: offset, height };
            self.amount_descending = write_at + 1;
        }
    }

    fn visible_with_threshold(&self, from: usize, threshold: u8) -> u32 {
        let mut viewing = self.amount_descending - 1;

        while viewing > 0 {
            if self.descending[viewing].height >= threshold {
                return (from - self.descending[viewing].position) as u32;
            }

            viewing -= 1;
        }

        return match self.descending[viewing].height >= threshold {
            true => from - self.descending[viewing].position,
            false => from - 0,
        } as u32;
    }
}

struct State {
    scores: [[u32; GRID_SIZE]; GRID_SIZE],
    highest_score: u32,
}


fn scan_trees(buffer: &[u8], direction: Direction, index: usize, visibility: &mut State) {
    let mut surveyor = BackwardSurveyor {
        highest: 0,
        descending: [Peak { position: 0, height: 0 }; 10],
        amount_descending: 0,
    };

    let (mut x, mut y) = match direction {
        Direction::UP => (index, GRID_SIZE - 1),
        Direction::DOWN => (index, 0),
        Direction::LEFT => (GRID_SIZE - 1, index),
        Direction::RIGHT => (0, index),
    };
    let mut offset = (GRID_SIZE + 1) * y + x;

    surveyor.move_forward(buffer[offset], 0);

    for distance in 1..GRID_SIZE-1 {

        match direction {
            Direction::UP => {
                offset -= GRID_SIZE + 1;
                y -= 1;
            },
            Direction::DOWN => {
                offset += GRID_SIZE + 1;
                y += 1;
            },
            Direction::LEFT => {
                offset -= 1;
                x -= 1;
            },
            Direction::RIGHT => {
                offset += 1;
                x += 1;
            },
        }

        let height = buffer[offset];

        visibility.scores[x][y] *= surveyor.visible_with_threshold(distance, height);
        if visibility.scores[x][y] > visibility.highest_score {
            visibility.highest_score = visibility.scores[x][y];
        }

        surveyor.move_forward(height, distance);
    }
}

pub(crate) fn part2_surveyor(input: &String) -> String {
    let buffer = input.as_bytes();

    let mut state = State {
        scores: [[1; GRID_SIZE]; GRID_SIZE],
        highest_score: 0,
    };

    for index in 1..GRID_SIZE-1 {
        scan_trees(buffer, Direction::LEFT, index, &mut state);
        scan_trees(buffer, Direction::RIGHT, index, &mut state);
        scan_trees(buffer, Direction::UP, index, &mut state);
        scan_trees(buffer, Direction::DOWN, index, &mut state);
    }

    return state.highest_score.to_string()
}

pub(crate) fn part2(input: &String) -> String {
    let buffer = input.as_bytes();

    let mut max_score = 0;

    for x in 1..GRID_SIZE-1 {
        for y in 1..GRID_SIZE-1 {
            let base_offset = (GRID_SIZE + 1) * y + x;
            let height = buffer[base_offset];

            let mut score = 1;
            let mut offset = base_offset;
            let mut i = x;
            let mut j = y;

            while i > 0 {
                i -= 1;
                offset -= 1;
                if buffer[offset] >= height {
                    break;
                }
            }

            score *= x - i;
            i = x;
            offset = base_offset;

            while i < GRID_SIZE-1 {
                i += 1;
                offset += 1;
                if buffer[offset] >= height {
                    break;
                }
            }

            score *= i - x;
            offset = base_offset;

            while j > 0 {
                j -= 1;
                offset -= GRID_SIZE + 1;
                if buffer[offset] >= height {
                    break;
                }
            }

            score *= y - j;
            j = y;
            offset = base_offset;

            while j < GRID_SIZE-1 {
                j += 1;
                offset += GRID_SIZE + 1;
                if buffer[offset] >= height {
                    break;
                }
            }

            score *= j - y;

            if score > max_score {
                max_score = score;
            }
        }
    }

    return max_score.to_string()
}