use super::SortAlgorithms;

pub fn gravity_sort<T>(arr: &mut Vec<T>) 
    where T: Copy + Clone + SortAlgorithms{
    let mut max = arr.iter().max_by_key(|a| a.get_key()).unwrap().get_key();
    let len = arr.len();
    let mut beads = vec![0; max as usize * len];
    let mut arr2 = arr.clone();

    for i in 0..len {
        for j in 0..arr[i].get_key() as usize {
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
        arr[i] = *arr2.iter().find(|a| a.get_key() == j as i32).unwrap();
    }
}
