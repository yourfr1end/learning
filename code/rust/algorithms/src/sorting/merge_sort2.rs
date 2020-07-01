pub fn merge_sort<T: Ord+Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        devide_and_merge(arr, 0, arr.len()-1)
    }
}

fn devide_and_merge<T: Ord+Copy>(arr: &mut [T], left: usize, rigth: usize) {
    if left < rigth {
        let mid = left + (rigth - left) / 2;
        devide_and_merge(arr, left, mid);
        devide_and_merge(arr, mid+1, rigth);
        merge(arr, left, mid, rigth)
    }
}

fn merge<T: Ord+Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let mut left_part = Vec::with_capacity(mid-left);
    let mut right_part = Vec::with_capacity(right-mid);
    for i in left..mid+1 {
        left_part.push(arr[i])
    }

    for i in mid+1..right+1 {
        right_part.push(arr[i])
    }

    let mut l_ptr = 0;
    let mut r_ptr = 0;
    let mut arr_ptr = left;


    while l_ptr < left_part.len() && r_ptr < right_part.len() {
        if left_part[l_ptr] < right_part[r_ptr] {
            arr[arr_ptr] = left_part[l_ptr];
            l_ptr += 1;
        } else {
            arr[arr_ptr] = right_part[r_ptr];
            r_ptr += 1;
        }

        arr_ptr += 1;
    }

    while l_ptr < left_part.len() {
        arr[arr_ptr] = left_part[l_ptr];
        l_ptr += 1;
        arr_ptr += 1;
    }

    while r_ptr < right_part.len() {
        arr[arr_ptr] = right_part[r_ptr];
        r_ptr += 1;
        arr_ptr += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use super::super::is_sorted;

    #[test]
    fn merge_sort_test() {
        let mut arr = vec![1,6,4,1,2,-100, 102, 12, 124, 120, -12341, 0];

        merge_sort(&mut arr);
        is_sorted(&arr);
    }
}