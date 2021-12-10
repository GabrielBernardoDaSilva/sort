#[cfg(test)]
mod merge_sort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::merge_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Merge sort is an algorithm that use the strategy order by comparation and divide to conquer.
# Examples

```
extern crate sort_algorithms;
use sort_algorithms::merge_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
merge_sort(&mut arr, |a, b| a < b);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn merge_sort<T>(arr: &mut Vec<T>, compare: fn(T, T) -> bool)
where
    T: Copy + Clone,
{
    let begin: usize = 0;
    let end: usize = arr.len() - 1;
    mergesort(arr, begin, end, compare);
}

fn mergesort<T>(arr: &mut Vec<T>, begin: usize, end: usize, compare: fn(T, T) -> bool)
where
    T: Copy + Clone,
{
    if begin < end {
        let middle: usize = (end + begin) / 2;

        mergesort(arr, begin, middle, compare);
        mergesort(arr, middle + 1, end, compare);
        merge(arr, begin, middle, end, compare);
    }
}

fn merge<T>(arr: &mut Vec<T>, begin: usize, middle: usize, end: usize, compare: fn(T, T) -> bool)
where
    T: Copy + Clone,
{
    let mut init1 = begin;
    let mut init2 = middle + 1;

    let mut aux_arr = Vec::<T>::new();

    while init1 <= middle && init2 <= end {
        if compare(arr[init1], arr[init2]) {
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
