use std::fmt::Debug;

use crate::graphic_for_sort::bars::Bar;

pub fn sort(arr: &mut Vec<Bar>) {
    let begin: usize = 0;
    let end: usize = arr.len() - 1;
    mergesort(arr, begin, end);
}

fn mergesort(arr: &mut Vec<Bar>, begin: usize, end: usize) {
    if begin < end {
        let middle: usize = (end + begin) / 2;

        mergesort(arr, begin, middle);
        mergesort(arr, middle + 1, end);
        merge(arr, begin, middle, end);
    }
}

fn merge(arr: &mut Vec<Bar>, begin: usize, middle: usize, end: usize) {
    let mut init1 = begin;
    let mut init2 = middle + 1;
    let mut size = end - begin + 1;

    let mut aux_arr = Vec::<Bar>::new();

    while init1 <= middle && init2 <= end {
        if arr[init1] < arr[init2] {
            aux_arr.push(arr[init1]);
            init1 += 1;
        } else {
            aux_arr.push(arr[init2]);
            init2 += 1;
        }
    }

    while init1 <= middle {
        aux_arr.push(arr[init1]);
        init1 += 1;
    }

    while init2 <= end {
        aux_arr.push(arr[init2]);
        init2 += 1;
    }

    for pos in begin..=end {
        let aux = arr[pos];
        arr[pos] = aux_arr[pos - begin];
        arr[pos].pos_x = aux.pos_x;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
