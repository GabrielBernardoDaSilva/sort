#[cfg(test)]
mod insertion_sort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::insertion_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Insertion sort is an algorithm that use strategy where catch one element and compare against orthers.
# Examples

```
extern crate sort_algorithms;
use sort_algorithms::insertion_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
insertion_sort(&mut arr, |a, b| a < b);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn insertion_sort<T>(arr: &mut Vec<T>, compare: fn(&T, &T) -> bool) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
