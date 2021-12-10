#[cfg(test)]
mod radix_sort_test {

    #[test]
    fn sort() {
        //!important only positive keys
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
        super::radix_sort(&mut arr, |&a| a);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Radix sort is an algorithm that use the strategy non-comparative.
!important only positive keys
# Examples

```
extern crate sort_algorithms;
use sort_algorithms::radix_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
radix_sort(&mut arr, |&a| a);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn radix_sort<T>(arr: &mut Vec<T>, get_key: fn(&T) -> i32)
where
    T: Copy + Clone,
{
    let bigger: i32 = get_key(arr.iter().max_by_key(|a| get_key(a)).unwrap());
    let mut exp = 1;
    let len = arr.len();
    let arr2 = arr.clone();

    while bigger / exp > 0 {
        let mut bucket = [0; 10];
        for i in arr2.clone() {
            bucket[((get_key(&i) / exp) % len as i32) as usize] += 1;
        }
        for i in 1..len {
            bucket[i] += bucket[i - 1];
        }
        for i in (0..len).rev() {
            let pos = bucket[((get_key(&arr2[i]) / exp) % len as i32) as usize] - 1;
            arr[pos] = arr2[i];
        }

        exp *= len as i32;
    }
}
