#[cfg(test)]
mod heapsort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::heapsort(&mut arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Heap sort is an generalist algorithm that use the strategy order by selecion.
# Examples

```
extern crate sort_algorithms;
use sort_algorithms::heapsort;


let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
heapsort(&mut arr);
assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn heapsort<T>(arr: &mut Vec<T>)
where
    T: Copy + Clone + PartialEq + PartialOrd,
{
    let end = arr.len();
    for start in (0..end / 2).rev() {
        sift_down(arr, start, end - 1);
    }

    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down<T>(arr: &mut Vec<T>, start: usize, end: usize)
where
    T: Copy + Clone + PartialEq + PartialOrd,
{
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }

        if child < end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[root] < arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
