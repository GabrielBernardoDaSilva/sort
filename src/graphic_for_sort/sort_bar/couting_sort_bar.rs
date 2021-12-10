use ggez::graphics::Color;

use crate::graphic_for_sort::bars::Bar;


pub fn counting_sort_bar(arr: &mut Vec<Bar>)
{
    let max: usize = arr
        .iter()
        .max_by_key(|a| a.id)
        .unwrap()
        .id as usize;
    let min: usize = arr
        .iter()
        .min_by_key(|a| a.id)
        .unwrap()
        .id as usize;

    let mut prefix_sums = {
        let len = (max - min) + 1;
        let mut count_arr = Vec::with_capacity(len);
        count_arr.resize(len, 0);

        for value in arr.iter_mut() {
            let color = value.color;
            value.color = Color::WHITE;
            count_arr[value.id as usize] += 1;
            std::thread::sleep(std::time::Duration::from_millis(16));
            value.color = color;


        }

        count_arr
            .into_iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state - x)
            })
            .collect::<Vec<usize>>()
    };

    for value in arr.clone() {
        let index = value.id as usize;
        let temp = arr[prefix_sums[index]];


        arr[prefix_sums[index]] = value.clone();
        arr[prefix_sums[index]].pos_x = temp.pos_x;
        prefix_sums[index] += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        
        
    }
}
