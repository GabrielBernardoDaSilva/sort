pub fn insertion_sort<T>(arr: &mut Vec<T>, compare: fn(&T, &T) -> bool) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
            
        }
    }
}
