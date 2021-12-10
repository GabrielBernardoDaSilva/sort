use super::SortAlgorithms;

pub fn flash_sort<T>(arr: &mut Vec<T>) 
    where T: SortAlgorithms + Copy + Clone{
    let mut max = arr.iter().enumerate().max_by_key(|a| a.1.get_key()).unwrap().0;
    let mut min = arr.iter().min_by_key(|a| a.get_key()).unwrap().get_key();
    let mut k = 0.0;

    let n = arr.len();
    let m = (0.45 * n as f32).floor() as i32;

    let mut l = vec![0 as i32; m as usize];

    if min == arr[max].get_key() {
        return;
    }

    let c1 = (m - 1) as f32 / (arr[max].get_key() - min) as f32;

    for k in 0..m as usize {
        l[k] = 0;
    }

    for j in 0..n {
        k = (c1 * (arr[j].get_key() - min) as f32).floor();
        l[k as usize] += 1;
    }

    for p in 1..m as usize {
        l[p] = l[p] + l[p - 1];
    }

    let mut hold = arr[max];
    arr[max] = arr[0];
    arr[0] = hold;

    let mut nmove = 0;
    let mut t = 0;
    let mut j = 0;
    k = m as f32 - 1.0;


    while nmove < (n - 1) {
        while j > (l[k as usize] - 1) {
            j += 1;
            k = (c1 * (arr[j as usize].get_key() - min) as f32).floor();
        }

        if k < 0.0 {
            break;
        }
        let mut flash = arr[j as usize];
        while j != l[k as usize] {
            k = (c1 * (flash.get_key() - min) as f32).floor();
            t = l[k as usize] - 1;
            l[k as usize] = l[k as usize] - 1;

            hold = arr[t as usize];
            arr[t as usize] = flash;
            flash = hold;
            nmove += 1;
        }
    }

    for j in 1..n {
        hold = arr[j];
        let mut i: i32 = j as i32 - 1;
        while i >= 0 && arr[i as usize].get_key() > hold.get_key() {
            arr[i as usize + 1] = arr[i as usize];
            i -= 1;
        }
        arr[(i + 1) as usize] = hold;
    }

    
}
