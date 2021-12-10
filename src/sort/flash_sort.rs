#[cfg(test)]
mod flash_sort_test {

    #[test]
    fn sort() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
        super::flash_sort(&mut arr, |&a| a);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

/**
Flash Sort is an algorithm that use the strategy that you can compute the approximate final position directly from the element value,
    with no comparisons.

# Examples

```

extern crate sort_algorithms;
use sort_algorithms::flash_sort;


let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0, -1];
flash_sort(&mut arr, |&a| a);
assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6, 7]);
```
*/

pub fn flash_sort<T>(arr: &mut Vec<T>, get_key: fn(&T) -> i32)
where
    T: Copy + Clone,
{
    let max = arr
        .iter()
        .enumerate()
        .max_by_key(|a| get_key(a.1))
        .unwrap()
        .0;
    let min = get_key(arr.iter().min_by_key(|a| get_key(a)).unwrap());
    let mut k: f32;

    let n = arr.len();
    let m = (0.45 * n as f32).floor() as i32;

    let mut l = vec![0 as i32; m as usize];

    if min == get_key(&arr[max]) {
        return;
    }

    let c1 = (m - 1) as f32 / (get_key(&arr[max]) - min) as f32;

    for k in 0..m as usize {
        l[k] = 0;
    }

    for j in 0..n {
        k = (c1 * (get_key(&arr[j]) - min) as f32).floor();
        l[k as usize] += 1;
    }

    for p in 1..m as usize {
        l[p] = l[p] + l[p - 1];
    }

    let mut hold = arr[max];
    arr[max] = arr[0];
    arr[0] = hold;

    let mut nmove = 0;
    let mut t: i32;
    let mut j = 0;
    k = m as f32 - 1.0;

    while nmove < (n - 1) {
        while j > (l[k as usize] - 1) {
            j += 1;
            k = (c1 * (get_key(&arr[j as usize]) - min) as f32).floor();
        }

        if k < 0.0 {
            break;
        }
        let mut flash = arr[j as usize];
        while j != l[k as usize] {
            k = (c1 * (get_key(&flash) - min) as f32).floor();
            t = l[k as usize] - 1;
            l[k as usize] = l[k as usize] - 1;

            hold = arr[t as usize];
            arr[t as usize] = flash;
            flash = hold;
            nmove += 1;
        }
    }

    for j in 1..n {
        hold = arr[j];
        let mut i: i32 = j as i32 - 1;
        while i >= 0 && get_key(&arr[i as usize]) > get_key(&hold) {
            arr[i as usize + 1] = arr[i as usize];
            i -= 1;
        }
        arr[(i + 1) as usize] = hold;
    }
}
