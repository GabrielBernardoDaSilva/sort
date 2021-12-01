use std::thread;

use crate::graphic_for_sort::{bars::Bar, main_state::MainState};

pub fn quick_sort_bar(arr: &mut Vec<Bar>, start: usize, end: usize) {
   
    if start >= end {
        return;
    }
    let p = partion_bar(arr, start, end);

    quick_sort_bar(arr, start, if p != 0 { p - 1 } else { p } as usize);
    quick_sort_bar(arr, (p + 1) as usize, end);
}

fn partion_bar(arr: &mut Vec<Bar>, start: usize, end: usize) -> i64 {
    let mut pivot = arr[end];

    let mut index = start;

    let mut i = start;

    while i < end {
        if arr[i] < pivot {
            swap_arr_bar(arr, i, index);
            index += 1;
        }
        thread::sleep(std::time::Duration::from_millis(16));
        i += 1;
    }
    swap_arr_bar(arr, index, end);
    thread::sleep(std::time::Duration::from_millis(16));
    index as i64
}

fn swap_arr_bar(arr: &mut Vec<Bar>, i: usize, j: usize) {
    let temp_i = arr[i];
    let temp_j = arr[j];

    arr[i] = temp_j;
    arr[i].pos_x = temp_i.pos_x;

    arr[j] = temp_i;
    arr[j].pos_x = temp_j.pos_x;
}
