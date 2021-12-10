#[cfg(test)]
mod bubble_sort_test {
    use crate::bubble_sort;

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        bubble_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
/**
Bubble sort is an algorithm that use order by comapare values.

# Examples

```
extern crate sort_algorithms;
use sort_algorithms::bubble_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
bubble_sort(&mut arr, |a, b| a < b);
assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn bubble_sort<T>(arr: &mut Vec<T>, compare: fn(T, T) -> bool)
where
    T: Copy,
{
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if compare(arr[j], arr[i]) {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}
