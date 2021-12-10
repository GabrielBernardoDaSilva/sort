pub trait CoutingSort {
    fn get_identifier(&self) -> i32;
}

pub fn counting_sort<T>(arr: &mut Vec<T>)
where
    T: CoutingSort + Clone,
{
    let max: usize = arr
        .iter()
        .max_by_key(|a| a.get_identifier())
        .unwrap()
        .get_identifier() as usize;
    let min: usize = arr
        .iter()
        .min_by_key(|a| a.get_identifier())
        .unwrap()
        .get_identifier() as usize;

    let mut prefix_sums = {
        let len = (max - min) + 1;
        let mut count_arr = Vec::with_capacity(len);
        count_arr.resize(len, 0);

        for value in arr.iter() {
            count_arr[value.get_identifier() as usize] += 1;
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
        let index = value.get_identifier() as usize;
        arr[prefix_sums[index]] = value.clone();
        prefix_sums[index] += 1;
    }
}