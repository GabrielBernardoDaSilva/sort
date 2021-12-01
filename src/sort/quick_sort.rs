use crate::graphic_for_sort::bars::Bar;

fn partion<T>(arr: &mut Vec<T>, start: usize, end: usize) -> i64
where
    T: Copy + Clone + PartialOrd + PartialEq,
{
    let mut pivot = arr[end];

    let mut index = start;

    let mut i = start;

    while i < end {
        if arr[i] < pivot {
            swap_arr(arr, i, index);
            index += 1;
        }
        i += 1;
    }
    swap_arr(arr, index, end);
    index as i64
}

pub fn quick_sort<T>(arr: &mut Vec<T>, start: usize, end: usize)
where
    T: Copy + Clone + PartialEq + PartialOrd,
{
    if start >= end {
        return;
    }
    let p = partion(arr, start, end);

    quick_sort(arr, start, if p != 0 { p - 1 } else { p } as usize);
    quick_sort(arr, (p + 1) as usize, end);
}

fn swap_arr<T>(arr: &mut Vec<T>, i: usize, j: usize)
where
    T: Copy + Clone,
{
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}



