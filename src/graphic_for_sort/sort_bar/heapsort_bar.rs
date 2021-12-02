use std::thread;

use crate::graphic_for_sort::bars::Bar;

use super::swap_arr_bar;

pub fn heapsort(arr: &mut Vec<Bar>) {
    let end = arr.len();
    for start in (0..end / 2).rev() {
        sift_down(arr, start, end - 1);
    }

    for end in (1..arr.len()).rev() {
        swap_arr_bar(arr, end, 0);
        thread::sleep(std::time::Duration::from_millis(16));
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down(arr: &mut Vec<Bar>, start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }

        if child < end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[root] < arr[child] {
            swap_arr_bar(arr, root, child);
            thread::sleep(std::time::Duration::from_millis(16));
            root = child;
        } else {
            break;
        }
    }
}
