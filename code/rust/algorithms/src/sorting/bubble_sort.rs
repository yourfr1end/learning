pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        for j in 0..arr.len()-i-1 {
            if arr[j] > arr[j + 1] { arr.swap(j, j+1) }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::bubble_sort;
    use super::super::is_sorted;

    #[test]
    fn bubble_sort_test() {
        let mut vec_to_sort = vec![9,8,7,6,1231,452354,134134,5,-1,-123,-9867987,4,3,3,3,1];

        bubble_sort(&mut vec_to_sort);
        is_sorted(&vec_to_sort);
    }
}