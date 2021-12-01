use crate::graphic_for_sort::{bars::Bar, main_state::MainState, util};

pub fn bubble_sort<T>(arr: &mut Vec<T>, compare: fn(T, T) -> bool)
where
    T: Copy,
{
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if compare(arr[j], arr[i]) {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}
