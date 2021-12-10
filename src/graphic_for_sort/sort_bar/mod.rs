use super::bars::Bar;

pub mod quick_sort_bar;
pub mod bubble_sort_bar;
pub mod selection_sort_bar;
pub mod heapsort_bar;
pub mod merge_sort_bar;
pub mod radix_sort_bar;
pub mod insertion_sort_bar;
pub mod cocktail_shaker_sort_bar;
pub mod gravity_sort_bar;
pub mod couting_sort_bar;
pub mod flash_sort_bar;

pub fn swap_arr_bar(arr: &mut Vec<Bar>, i: usize, j: usize) {
    let temp_i = arr[i];
    let temp_j = arr[j];

    arr[i] = temp_j;
    arr[i].pos_x = temp_i.pos_x;

    arr[j] = temp_i;
    arr[j].pos_x = temp_j.pos_x;
}
