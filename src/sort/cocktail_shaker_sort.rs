pub fn cocktail_shaker_sort(arr: &mut Vec<i32>) {
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
                
            }
        }
        if !swapped {
            break;
        }
        swapped = false;
        for i in (0..arr.len() - 1).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
                
            }
        }

        if !swapped {
            break;
        }
    }
}
