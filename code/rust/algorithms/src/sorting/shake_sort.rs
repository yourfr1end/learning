pub fn shake_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        _shake_sort(arr)
    }
}

fn _shake_sort<T: Ord>(arr: &mut [T]) {
    let mut left = 0;
    let mut right = arr.len()-1;

    while left < right {
        for i in left..right {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
            }
        }
        right -= 1;

        for i in (left+1..=right).rev() {
            if arr[i-1] > arr[i] {
                arr.swap(i-1, i);
            }
        }
        left += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::shake_sort;
    use super::super::is_sorted;

    #[test]
    fn shake_sort_test() {
        let mut arr = vec![5,1,2,6,3,-7,9,-8,-4,-2139];
        shake_sort(&mut arr);

        is_sorted(&arr);
    }
}
