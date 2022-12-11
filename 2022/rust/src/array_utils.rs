use std::ops::{AddAssign, MulAssign};

pub(crate) fn top_n<T: Ord + Copy>(array: &mut [T], top_amount: usize) {
    let mut cursor = 0;
    for i in 1..array.len() {
        let mut current = array[i];
        if current > array[top_amount.min(i) - 1] {
            while cursor < top_amount.min(i + 1) {
                if current > array[cursor] {
                    let previous_max = array[cursor];
                    array[cursor] = current;
                    current = previous_max;
                }
                cursor += 1;
            }
            cursor = 0;
        }
    }
}

pub(crate) fn product_n<T: Copy + MulAssign<T>>(array: &[T], amount: usize) -> T {
    let mut product = array[0];
    for i in 1..amount {
        product *= array[i];
    }
    return product;
}

#[allow(dead_code)]
pub(crate) fn sum_n<T: Copy + AddAssign<T>>(array: &[T], amount: usize) -> T {
    let mut product = array[0];
    for i in 1..amount {
        product += array[i];
    }
    return product;
}
