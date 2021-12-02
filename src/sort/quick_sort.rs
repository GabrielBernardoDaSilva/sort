use crate::graphic_for_sort::bars::Bar;

fn partion<T>(arr: &mut Vec<T>, start: usize, end: usize, compare: fn(T, T) -> bool) -> i64
where
    T: Copy + Clone + PartialOrd + PartialEq,
{
    let mut pivot = arr[end];

    let mut index = start;

    let mut i = start;

    while i < end {
        if compare(arr[i], pivot) {
            arr.swap(i, index);
            index += 1;
        }
        i += 1;
    }
    arr.swap(index, end);
    index as i64
}

pub fn quick_sort<T>(arr: &mut Vec<T>, start: usize, end: usize, compare: fn (T, T) -> bool)
where
    T: Copy + Clone + PartialEq + PartialOrd,
{
    if start >= end {
        return;
    }
    let p = partion(arr, start, end, compare);

    quick_sort(arr, start, if p != 0 { p - 1 } else { p } as usize, compare);
    quick_sort(arr, (p + 1) as usize, end, compare);
}
