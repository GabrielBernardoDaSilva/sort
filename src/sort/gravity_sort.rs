#[cfg(test)]
mod gravity_sort_test {

    #[test]
    fn sort() {
        //!important only positive keys
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
        super::gravity_sort(&mut arr, |&a| a);
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
use sort_algorithms::gravity_sort;


let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
gravity_sort(&mut arr, |&a| a);
assert_eq!(arr, [ 0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn gravity_sort<T>(arr: &mut Vec<T>, get_key: fn(&T) -> i32)
where
    T: Copy + Clone,
{
    let max = get_key(arr.iter().max_by_key(|a| get_key(a)).unwrap());
    let len = arr.len();
    let mut beads = vec![0; max as usize * len];
    let arr2 = arr.clone();

    println!("pass");
    for i in 0..len {
        for j in 0..get_key(&arr[i]) as usize {
            let pos = i * (max as usize) + j;
            beads[pos] = 1;
        }
    }

    for j in 0..max as usize {
        let mut sum = 0;
        for i in 0..len {
            sum += beads[i * (max as usize) + j];
            beads[i * (max as usize) + j] = 0;
        }

        for i in (len - sum)..len {
            beads[i * (max as usize) + j] = 1;
        }
    }

    for i in 0..len {
        let mut j: usize = 0;

        while j < max as usize && beads[i * (max as usize) + j] == 1 {
            j += 1;
        }
        // important increasse time here for work with any type that could be indexed
        arr[i] = *arr2.iter().find(|a| get_key(a) == j as i32).unwrap();
    }
}
