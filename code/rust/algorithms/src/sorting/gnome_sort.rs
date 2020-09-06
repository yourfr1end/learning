pub fn gnome_sort<T: Ord>(arr: &mut [T]) {
    let mut i = 1;
    let mut j = 2;
    while i < arr.len() {
        if arr[i] > arr[i-1] {
            i = j;
            j += 1;
        } else {
            arr.swap(i, i-1);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gnome_sort;
    use super::super::is_sorted;

    #[test]
    fn gnome_sort_test() {
        let mut arr = vec![4,2,1,4,2,123124,0,-1,-9999];

        gnome_sort(&mut arr);
        is_sorted(&arr);
    }
}