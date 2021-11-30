use crate::graphic_for_sort::{bars::Bar, main_state::MainState, util};

pub fn bubble_sort(arr: &mut Vec<Bar>, compare: fn(Bar, Bar) -> bool) {
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if compare(arr[i], arr[j]) {
                let temp_i = arr[i];
                let temp_j = arr[j];
                arr[i] = arr[j];
                arr[i].pos_x = temp_i.pos_x;

                arr[j] = temp_i;
                arr[j].pos_x = temp_j.pos_x;
            }
        }
    }
}

pub fn bubble_sort_visualizer(main_state: &mut MainState) {
    if main_state.control_index.i < main_state.bars.len() {
        if main_state.control_index.j < main_state.bars.len() {
            if main_state.bars[main_state.control_index.i].id
                < main_state.bars[main_state.control_index.j].id
            {
                let temp_i = main_state.bars[main_state.control_index.i];
                let temp_j = main_state.bars[main_state.control_index.j];
                main_state.bars[main_state.control_index.i] =
                    main_state.bars[main_state.control_index.j];
                main_state.bars[main_state.control_index.i].pos_x = temp_i.pos_x;

                main_state.bars[main_state.control_index.j] = temp_i;
                main_state.bars[main_state.control_index.j].pos_x = temp_j.pos_x;
            }
            main_state.control_index.j += 1;
        } else {
            main_state.control_index.j = 0;
            main_state.control_index.i += 1;
        }
    } else {
        main_state.trigger_sort = false;
    }
}
