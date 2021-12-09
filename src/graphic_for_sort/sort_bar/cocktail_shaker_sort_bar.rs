use crate::graphic_for_sort::bars::Bar;

use super::swap_arr_bar;

pub fn cocktail_shaker_sort_bar(arr: &mut Vec<Bar>) {
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                swap_arr_bar(arr, i, i + 1);               
                swapped = true;
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
        if !swapped {
            break;
        }
        swapped = false;
        for i in (0..arr.len() - 1).rev() {
            if arr[i] > arr[i + 1] {
                swap_arr_bar(arr, i, i + 1);
                swapped = true;
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }

        if !swapped {
            break;
        }
    }
}
