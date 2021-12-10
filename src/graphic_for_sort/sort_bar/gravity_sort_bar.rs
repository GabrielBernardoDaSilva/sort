use crate::graphic_for_sort::bars::Bar;

pub fn gravity_sort_bar(arr: &mut Vec<Bar>) 
  {
    let mut max = arr.iter().max_by_key(|a| a.id).unwrap().id;
    let len = arr.len();
    let mut beads = vec![0; max as usize * len];
    let mut arr2 = arr.clone();

    for i in 0..len {
        for j in 0..arr[i].id as usize {
            beads[i * (max as usize) + j] = 1;
        }
    }

    for j in 0..max as usize {
        let mut sum = 0;
        for i in 0..len {
            sum += beads[i * (max as usize) + j];
            beads[i * (max as usize) + j] = 0;
        }

        for i in (len - sum)..len {
            beads[i * (max as usize) + j] = 1;
        }
    }

    for i in 0..len {
        let mut j: usize = 0;

        while j < max as usize && beads[i * (max as usize) + j] == 1 {
            j += 1;
        }
        // important increasse time here for work with any type that could be indexed
        let temp_i = arr[i];
        arr[i] = *arr2.iter().find(|a| a.id == j as i32).unwrap();
        arr[i].pos_x = temp_i.pos_x;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
