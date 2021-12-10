#[cfg(test)]
mod cocktail_shaker_sort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::cocktail_shaker_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Cocktail Shaker Sort is an algorithm is a derivation from bubble sort.

# Examples

```
extern crate sort_algorithms;
use sort_algorithms::cocktail_shaker_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
cocktail_shaker_sort(&mut arr, |a, b| a < b);
assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn cocktail_shaker_sort<T>(arr: &mut Vec<T>, compare: fn(&T, &T) -> bool) {
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if compare(&arr[i + 1], &arr[i]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        swapped = false;
        for i in (0..arr.len() - 1).rev() {
            if compare(&arr[i + 1], &arr[i]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
