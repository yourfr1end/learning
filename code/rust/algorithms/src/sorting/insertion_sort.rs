pub fn insertion_sort<T: Ord+Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let curr = arr[i];
        let mut j = i;

        while j > 0 && arr[j-1] > curr {
            arr[j] = arr[j-1];
            j -= 1;
        }

        arr[j] = curr;
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;
    use super::super::is_sorted;

    #[test]
    fn insertion_sort_test() {
        let mut arr = vec![8,-1000,7,6,-5,1,2,-3,-4,-99999];

        insertion_sort(&mut arr);
        is_sorted(&arr);
    }
}