use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ptr::{copy, write, NonNull, read};
use crate::parser::read_unsigned_int;

pub trait Inventory {
    fn crate_mover_9000(&mut self, amount: usize, from: usize, to: usize);
    fn crate_mover_9001(&mut self, amount: usize, from: usize, to: usize);
    fn set_crate(&mut self, stack: usize, height: usize, value: u8);
    fn get_top_crate(&self, stack: usize) -> u8;
}

const MAX_CRATE_STACK_HEIGHT: usize = 128;
const MAX_CRATE_STACKS: usize = 9;

struct MemoryInventory {
    ptr: NonNull<u8>,
    cap: usize,
    _marker: PhantomData<u8>,
    heights: [usize; MAX_CRATE_STACKS],
}

impl MemoryInventory {
    fn new(amount_stacks: usize) -> Self {
        let cap = amount_stacks * MAX_CRATE_STACK_HEIGHT;
        let layout = Layout::array::<u8>(cap).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        MemoryInventory {
            ptr: NonNull::new(unsafe { alloc(layout) } as *mut u8).unwrap(),
            cap,
            _marker: PhantomData,
            heights: [0; MAX_CRATE_STACKS],
        }
    }
}


impl Inventory for MemoryInventory {
    fn crate_mover_9000(&mut self, amount: usize, from: usize, to: usize) {
        for i in 0..amount {
            unsafe {
                copy(
                    self.ptr.as_ptr().add(from * MAX_CRATE_STACK_HEIGHT + self.heights[from] - i - 1),
                    self.ptr.as_ptr().add(to * MAX_CRATE_STACK_HEIGHT + self.heights[to] + i),
                    amount,
                );
            }
        }
        self.heights[from] -= amount;
        self.heights[to] += amount;
    }

    fn crate_mover_9001(&mut self, amount: usize, from: usize, to: usize) {
        unsafe {
            copy(
                self.ptr.as_ptr().add(from * MAX_CRATE_STACK_HEIGHT + self.heights[from] - amount),
                self.ptr.as_ptr().add(to * MAX_CRATE_STACK_HEIGHT + self.heights[to]),
                amount,
            );
        }
        self.heights[from] -= amount;
        self.heights[to] += amount;
    }

    fn set_crate(&mut self, stack: usize, height: usize, value: u8) {
        unsafe {
            write(self.ptr.as_ptr().add(stack * MAX_CRATE_STACK_HEIGHT + height), value);
        }
        if self.heights[stack] < height + 1 {
            self.heights[stack] = height + 1;
        }
    }

    fn get_top_crate(&self, stack: usize) -> u8 {
        unsafe {
            read(self.ptr.as_ptr().add(stack * MAX_CRATE_STACK_HEIGHT + self.heights[stack] - 1))
        }
    }
}

impl Drop for MemoryInventory {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<u8>(self.cap).unwrap(),
            );
        }
    }
}

#[derive(Copy, Clone)]
struct CrateStack {
    crates: [u8; MAX_CRATE_STACK_HEIGHT],
    height: usize,
}

struct StackInventory {
    stacks: [CrateStack; MAX_CRATE_STACKS],
}


impl StackInventory {
    fn new(amount_stacks: usize) -> Self {
        assert!(amount_stacks <= MAX_CRATE_STACKS, "Allocation too large");
        StackInventory {
            stacks: [CrateStack {crates: [0; MAX_CRATE_STACK_HEIGHT], height: 0}; MAX_CRATE_STACKS],
        }
    }
}


impl Inventory for StackInventory {
    fn crate_mover_9000(&mut self, amount: usize, from: usize, to: usize) {

        for i in 0..amount {
            self.stacks[to].crates[self.stacks[to].height + i] = self.stacks[from].crates[self.stacks[from].height - i - 1];
        }
        self.stacks[from].height -= amount;
        self.stacks[to].height += amount;
    }

    fn crate_mover_9001(&mut self, amount: usize, from: usize, to: usize) {
        for i in 0..amount {
            self.stacks[to].crates[self.stacks[to].height + i] = self.stacks[from].crates[self.stacks[from].height + i - amount];
        }
        self.stacks[from].height -= amount;
        self.stacks[to].height += amount;
    }

    fn set_crate(&mut self, stack: usize, height: usize, value: u8) {
        self.stacks[stack].crates[height] = value;
        if self.stacks[stack].height < height + 1 {
            self.stacks[stack].height = height + 1;
        }
    }

    fn get_top_crate(&self, stack: usize) -> u8 {
        self.stacks[stack].crates[self.stacks[stack].height - 1]
    }
}

fn get_amount_stacks(buffer: &[u8]) -> usize {
    let mut offset = 3;
    let mut stacks = 1;
    while buffer[offset] != '\n' as u8 {
        stacks += 1;
        offset += 4;
    }
    return stacks;
}

fn get_highest_stack(buffer: &[u8], amount_stacks: usize) -> usize {
    let mut offset = amount_stacks * 4;
    let mut height = 0;
    while buffer[offset] != '\n' as u8 {
        height += 1;
        offset += amount_stacks * 4;
    }
    return height;
}

fn parse_crate_stacks(buffer: &[u8], offset: &mut usize, inventory_generator: impl Fn(usize) -> Box<dyn Inventory>) -> Box<dyn Inventory> {
    let amount_stacks = get_amount_stacks(buffer);
    let highest_stack = get_highest_stack(buffer, amount_stacks);

    let mut inventory = inventory_generator(amount_stacks);

    let mut height = highest_stack - 1;
    *offset += 1;
    loop {
        for stack in 0..amount_stacks {
            if buffer[*offset] != ' ' as u8 {
                inventory.set_crate(stack, height, buffer[*offset]);
            }
            *offset += 4;
        }

        if height == 0 { break }
        height -= 1;
    }
    *offset += amount_stacks * 4;
    return inventory
}

fn parse_instruction(buffer: &[u8], offset: &mut usize) -> (usize, usize, usize) {
    *offset += 5;
    let amount = read_unsigned_int(buffer, offset).unwrap();
    *offset += 6;
    let from = read_unsigned_int(buffer, offset).unwrap() - 1;
    *offset += 4;
    let to = read_unsigned_int(buffer, offset).unwrap() - 1;
    *offset += 1;
    return (amount, from, to);
}

fn generate_stack_inventory(amount_stacks: usize) -> Box<dyn Inventory> {
    Box::new(StackInventory::new(amount_stacks))
}

fn generate_memory_inventory(amount_stacks: usize) -> Box<dyn Inventory> {
    Box::new(MemoryInventory::new(amount_stacks))
}


pub(crate) fn part1(buffer: &[u8], inventory_generator: impl Fn(usize) -> Box<dyn Inventory>) -> String {
    let mut offset = 0;

    let mut inventory = parse_crate_stacks(buffer, &mut offset, inventory_generator);
    while offset < buffer.len() {
        let (amount, from, to) = parse_instruction(buffer, &mut offset);
        inventory.crate_mover_9000(amount, from, to);
    }

    let mut top = String::with_capacity(MAX_CRATE_STACKS);
    for stack in 0..MAX_CRATE_STACKS {
        unsafe { top.as_mut_vec().push(inventory.get_top_crate(stack)) }
    }

    return top;
}

pub(crate) fn part1_memory(buffer: &[u8]) -> String {
    part1(buffer, &generate_memory_inventory)
}

pub(crate) fn part1_stack(buffer: &[u8]) -> String {
    part1(buffer, &generate_stack_inventory)
}

pub(crate) fn part2(buffer: &[u8], inventory_generator: impl Fn(usize) -> Box<dyn Inventory>) -> String {
    let mut offset = 0;

    let mut inventory = parse_crate_stacks(buffer, &mut offset, inventory_generator);
    while offset < buffer.len() {
        let (amount, from, to) = parse_instruction(buffer, &mut offset);
        inventory.crate_mover_9001(amount, from, to);
    }

    let mut top = String::with_capacity(MAX_CRATE_STACKS);
    for stack in 0..MAX_CRATE_STACKS {
        unsafe { top.as_mut_vec().push(inventory.get_top_crate(stack)) }
    }

    return top;
}

pub(crate) fn part2_memory(buffer: &[u8]) -> String {
    part2(buffer, &generate_memory_inventory)
}

pub(crate) fn part2_stack(buffer: &[u8]) -> String {
    part2(buffer, &generate_stack_inventory)
}
