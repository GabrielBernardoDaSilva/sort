use std::ptr::swap;

use crate::graphic_for_sort::{
    bars::Bar,
    main_state::{self, MainState},
};

pub fn selection_sort<T>(arr: &mut Vec<T>, compare: fn(T,T) -> bool)
where
    T:  Copy + Clone,
{
    for i in 0..arr.len() {
        let mut min = i;

        for j in i + 1..arr.len() {
            if compare(arr[j], arr[min]) {
                min = j;
            }
        }
        if min != i {
            let temp = arr[i];
            arr[i] = arr[min];
            arr[min] = temp;
        }
    }
}

