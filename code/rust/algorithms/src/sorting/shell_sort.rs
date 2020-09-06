pub fn shell_sort<T: Ord+Copy>(arr: &mut [T]) {
    let mut increment = arr.len() / 2;
    while increment > 0 {
        for start in 0..increment {
            gap_insertion_sort(arr, start, increment);
        }
        increment /= 2;
    }
}

fn gap_insertion_sort<T: Ord+Copy>(arr: &mut [T], low: usize, gap: usize) {
    for i in (low+gap..arr.len()).step_by(gap) {
        let current_value = arr[i];
        let mut position = i;

        while position >= gap && arr[position-gap] > current_value {
            arr[position] = arr[position - gap];
            position = position -gap;
        }

        arr[position] = current_value;
    }
}

#[cfg(test)]
mod tests {
    use super::shell_sort;
    use super::super::is_sorted;

    #[test]
    fn shake_sort_test() {
        let mut arr = vec![23,12,1,0,1,4,4,2,2,-4,-2139];
        shell_sort(&mut arr);

        is_sorted(&arr);
    }
}