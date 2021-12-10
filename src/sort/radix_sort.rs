use std::ops::{Div, DivAssign};

use super::SortAlgorithms;

pub fn radix_sort<T>(arr: &mut Vec<T>)
where
    T: Copy + Clone + SortAlgorithms,
{
    let mut bigger: i32 = arr
        .iter()
        .max_by_key(|a| a.get_key())
        .unwrap()
        .get_key();
    let mut exp = 1;
    let len = arr.len();
    let mut arr2 = arr.clone();

    while bigger / exp > 0 {
        let mut bucket = [0; 10];
        for i in arr2.clone() {
            bucket[((i.get_key() / exp) % len as i32) as usize] += 1;
        }
        for i in 1..len {
            bucket[i] += bucket[i - 1];
        }
        for i in (0..len).rev() {
            let pos = bucket[((arr2[i].get_key() / exp) % len as i32) as usize] - 1;
            arr[pos] = arr2[i];
        }

        exp *= len as i32;
    }
}
