pub fn selection_sort<T: Ord>(arr:&mut [T]) {
    for i in 0..arr.len() {
        let mut curr_idx = i;
        for j in i+1..arr.len() {
            if arr[j] < arr[curr_idx] {
                curr_idx = j;
            }
        }

        arr.swap(curr_idx, i);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;
    use super::super::is_sorted;

    #[test]
    fn selection_sort_test() {
        let mut arr = vec![4,2,1,4,2,123124,0,-1,-9999];

        selection_sort(&mut arr);
        is_sorted(&arr);
    }
}