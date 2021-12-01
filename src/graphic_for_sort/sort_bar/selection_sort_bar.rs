use crate::graphic_for_sort::{bars::Bar, main_state::MainState};

pub fn selection_sort_bar(arr: &mut Vec<Bar>) {
    for i in 0..arr.len() {
        let mut min = i;

        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        if min != i {
            std::thread::sleep(std::time::Duration::from_millis(60));
            let temp_i = arr[i];
            let temp_min = arr[min];

            arr[i] = temp_min;
            arr[i].pos_x = temp_i.pos_x;

            arr[min] = temp_i;
            arr[min].pos_x = temp_min.pos_x;
        }
    }
}

pub fn selection_sort_visualizer(main_state: &mut MainState) {
    if main_state.control_index.i < main_state.bars.len() {
       
        if main_state.control_index.j < main_state.bars.len() {

            if main_state.control_index.j == 0 {
                main_state.control_index.j = main_state.control_index.i + 1;
            }

            if main_state.bars[main_state.control_index.j].id
                < main_state.bars[main_state.min].id
            {
                main_state.min = main_state.control_index.j;
            }
            main_state.control_index.j += 1;

        } else {

            if main_state.min != main_state.control_index.i {
                let temp_i = main_state.bars[main_state.control_index.i];
                let temp_min = main_state.bars[main_state.min];

                main_state.bars[main_state.control_index.i] = temp_min;
                main_state.bars[main_state.control_index.i].pos_x = temp_i.pos_x;

                main_state.bars[main_state.min] = temp_i;
                main_state.bars[main_state.min].pos_x = temp_min.pos_x;
            }
            main_state.control_index.i += 1;
            main_state.control_index.j = main_state.control_index.i + 1;
            main_state.min = main_state.control_index.i;
        }
    } else {
        main_state.trigger_sort = false;
    }
}
