use std::fmt::Debug;

pub fn sort(arr: &mut Vec<i32>) {
    let begin: usize = 0;
    let end: usize = arr.len() - 1;
    mergesort(arr, begin, end);
}

fn mergesort(arr: &mut Vec<i32>, begin: usize, end: usize) {
    if begin < end {
        let middle: usize = (end + begin) / 2;

        mergesort(arr, begin, middle);
        mergesort(arr, middle + 1, end);
        merge(arr, begin, middle, end);
    }
}

fn merge(arr: &mut Vec<i32>, begin: usize, middle: usize, end: usize) {
    let mut init1 = begin;
    let mut init2 = middle + 1;
    let mut size = end - begin + 1;

    let mut aux_arr = Vec::<i32>::new();

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
        arr[pos] = aux_arr[pos - begin];
    }
}
