#[cfg(test)]
mod selection_sort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::selection_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Selection Sort is an algorithm that use order by selection.
!important only positive keys
# Examples

```
extern crate sort_algorithms;
use sort_algorithms::selection_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
selection_sort(&mut arr, |a, b| a < b);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn selection_sort<T>(arr: &mut Vec<T>, compare: fn(T, T) -> bool)
where
    T: Copy + Clone,
{
    for i in 0..arr.len() {
        let mut min = i;

        for j in i + 1..arr.len() {
            if compare(arr[j], arr[min]) {
                min = j;
            }
        }
        if min != i {
            let temp = arr[i];
            arr[i] = arr[min];
            arr[min] = temp;
        }
    }
}
