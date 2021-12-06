use crate::graphic_for_sort::bars::Bar;

pub fn radix_sort_bar(arr: &mut Vec<Bar>) {
    let mut bigger = arr.iter().max_by_key(|a| a.id).unwrap().id;
    let mut exp = 1;
    let len = arr.len();
    let mut arr2= arr.clone();

    while bigger / exp > 0 {
        let mut bucket = vec![0; len];
        for i in arr2.clone() {
            bucket[((i.id / exp) % len as i32) as usize] += 1;
        }
        for i in 1..len {
            bucket[i] += bucket[i - 1];
        }
        for i in (0..len).rev() {
            let pos = bucket[((arr2[i].id / exp) % len as i32) as usize] - 1;
            let aux = arr[pos];
            arr[pos] = arr2[i];
            arr[pos].pos_x = aux.pos_x;
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        exp *= len as i32;
    }
    println!("arr 2:{:?}", &arr2);
}
