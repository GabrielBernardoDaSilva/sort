#[cfg(test)]
mod counting_sort_test {

    #[test]
    fn sort() {
        //!important only positive keys
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
        super::counting_sort(&mut arr, |&a| a);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Counting Sort is an algorithm that use the strategy of  it uses key values as indexes into an array and
    the lower bound for comparison sorting will not apply.
!important only positive keys
# Examples

```

extern crate sort_algorithms;
use sort_algorithms::counting_sort;

let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
counting_sort(&mut arr, |&a| a);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn counting_sort<T>(arr: &mut Vec<T>, get_key: fn(&T) -> i32)
where
    T: Clone,
{
    let max: usize = get_key(arr.iter().max_by_key(|a| get_key(a)).unwrap()) as usize;
    let min: usize = get_key(arr.iter().min_by_key(|a| get_key(a)).unwrap()) as usize;

    let mut prefix_sums = {
        let len = (max - min) + 1;
        let mut count_arr = Vec::with_capacity(len);
        count_arr.resize(len, 0);

        for value in arr.iter() {
            count_arr[get_key(value) as usize] += 1;
        }

        count_arr
            .into_iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state - x)
            })
            .collect::<Vec<usize>>()
    };

    for value in arr.clone() {
        let index = get_key(&value) as usize;
        arr[prefix_sums[index]] = value.clone();
        prefix_sums[index] += 1;
    }
}
