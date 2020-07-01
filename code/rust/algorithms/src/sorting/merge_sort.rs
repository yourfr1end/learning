pub fn merge_sort<T: Ord+Copy>(arr: &mut [T]) {
    let sorted = _merge_sort(arr);
    for (i, e) in sorted.iter().enumerate() {
        arr[i] = *e;
    }
}

fn _merge_sort<T: Ord+Copy>(arr: &mut [T]) -> Vec<T> {
    let len = arr.len();
    if len == 1 {
        return vec![arr[0]];
    }

    let mid = len / 2;
    merge(_merge_sort(&mut arr[0..mid]), _merge_sort(&mut arr[mid..len]))
}

fn merge<T: Ord+Copy>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let l_size = left.len();
    let r_size = right.len();
    let mut result = Vec::with_capacity(l_size + r_size);

    let mut l_ptr = 0;
    let mut r_ptr = 0;

    while l_ptr < l_size && r_ptr < r_size {
        if left[l_ptr] < right[r_ptr] {
            result.push(left[l_ptr]);
            l_ptr += 1;
        } else {
            result.push(right[r_ptr]);
            r_ptr += 1;
        }
    }

    while l_ptr < l_size {
        result.push(left[l_ptr]);
        l_ptr += 1;
    }
    

    while r_ptr < r_size {
        result.push(right[r_ptr]);
        r_ptr += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use super::super::is_sorted;

    #[test]
    fn merge_sort_test() {
        let mut arr = vec![8,7,6,-5,1,2,-3,-4];

        merge_sort(&mut arr);
        is_sorted(&arr);
    }
}