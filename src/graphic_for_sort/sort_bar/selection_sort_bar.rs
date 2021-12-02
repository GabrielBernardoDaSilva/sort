use crate::graphic_for_sort::{bars::Bar, main_state::MainState};

use super::swap_arr_bar;

pub fn selection_sort_bar(arr: &mut Vec<Bar>) {
    for i in 0..arr.len() {
        let mut min = i;

        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        if min != i {
            std::thread::sleep(std::time::Duration::from_millis(60));
            swap_arr_bar(arr, i, min);
        }
    }
}
