#[cfg(test)]
mod quick_sort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::quick_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Quick sort is an algorithm that use strategy divide to conquer.
# Examples

```
extern crate sort_algorithms;
use sort_algorithms::quick_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
quick_sort(&mut arr, |a, b| a < b);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

fn partion<T>(arr: &mut Vec<T>, start: usize, end: usize, compare: fn(T, T) -> bool) -> i64
where
    T: Copy + Clone,
{
    let pivot = arr[end];

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

fn quick_sort_rec<T>(arr: &mut Vec<T>, start: usize, end: usize, compare: fn(T, T) -> bool)
where
    T: Copy + Clone,
{
    if start >= end {
        return;
    }
    let p = partion(arr, start, end, compare);

    quick_sort_rec(arr, start, if p != 0 { p - 1 } else { p } as usize, compare);
    quick_sort_rec(arr, (p + 1) as usize, end, compare);
}

pub fn quick_sort<T>(arr: &mut Vec<T>, compare: fn(T, T) -> bool)
where
    T: Copy + Clone,
{
    let start: usize = 0;
    let end = arr.len() - 1;

    quick_sort_rec(arr, start, end, compare);
}
