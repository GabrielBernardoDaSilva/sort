pub fn cocktail_shaker_sort<T>(arr: &mut Vec<T>, compare: fn(&T, &T) -> bool) {
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if compare(&arr[i], &arr[i + 1]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        swapped = false;
        for i in (0..arr.len() - 1).rev() {
            if compare(&arr[i], &arr[i + 1]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
