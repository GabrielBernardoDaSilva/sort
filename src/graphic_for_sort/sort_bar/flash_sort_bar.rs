use crate::graphic_for_sort::bars::Bar;

pub fn flash_sort_bar(arr: &mut Vec<Bar>) {
    let mut max = arr.iter().enumerate().max_by_key(|a| a.1.id).unwrap().0;
    let mut min = arr.iter().min_by_key(|a| a.id).unwrap().id;
    let mut k = 0.0;

    let n = arr.len();
    let m = (0.45 * n as f32).floor() as i32;

    let mut l = vec![0 as i32; m as usize];

    if min == arr[max].id {
        return;
    }

    let c1 = (m - 1) as f32 / (arr[max].id - min) as f32;

    for k in 0..m as usize {
        l[k] = 0;
    }

    for j in 0..n {
        k = (c1 * (arr[j].id - min) as f32).floor();
        l[k as usize] += 1;
    }

    for p in 1..m as usize {
        l[p] = l[p] + l[p - 1];
    }
    //swap
    let mut hold = arr[max];
    let temp = arr[0];

    arr[max] = arr[0];
    arr[max].pos_x = hold.pos_x;

    arr[0] = hold;
    arr[0].pos_x = temp.pos_x;

    std::thread::sleep(std::time::Duration::from_millis(16));


    let mut nmove = 0;
    let mut t = 0;
    let mut j = 0;
    k = m as f32 - 1.0;


    while nmove < (n - 1) {
        while j > (l[k as usize] - 1) {
            j += 1;
            k = (c1 * (arr[j as usize].id - min) as f32).floor();
        }

        if k < 0.0 {
            break;
        }
        let mut flash = arr[j as usize];
        while j != l[k as usize] {
            k = (c1 * (flash.id - min) as f32).floor();
            t = l[k as usize] - 1;
            l[k as usize] = l[k as usize] - 1;
            //swap
            hold = arr[t as usize];
            arr[t as usize] = flash;
            flash = hold;
            arr[t as usize].pos_x = hold.pos_x;

            std::thread::sleep(std::time::Duration::from_millis(16));

            nmove += 1;
        }
    }
    for j in 1..n {
        hold = arr[j];
        let mut i: i32 = j as i32 - 1;
        while i >= 0 && arr[i as usize].id > hold.id {
            //swap
            let temp_pos_x = arr[i as usize + 1].pos_x;
            arr[i as usize + 1] = arr[i as usize];
            arr[i as usize + 1].pos_x = temp_pos_x;
            std::thread::sleep(std::time::Duration::from_millis(16));
            i -= 1;
        }
        //swap
        let temp_pos_x = arr[(i + 1) as usize].pos_x;
        arr[(i + 1) as usize] = hold;
        arr[(i + 1) as usize].pos_x = temp_pos_x;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
