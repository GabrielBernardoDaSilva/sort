use ggez::graphics::Color;

use crate::graphic_for_sort::bars::Bar;

use super::swap_arr_bar;

pub fn insertion_sort_bar(arr: &mut Vec<Bar>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            swap_arr_bar(arr, j, j - 1);
            j -= 1;
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
