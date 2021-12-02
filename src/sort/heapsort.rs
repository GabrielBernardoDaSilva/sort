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
