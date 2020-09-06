mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod merge_sort2;
mod shake_sort;
mod gnome_sort;
mod selection_sort;
mod shell_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::merge_sort;
pub use self::merge_sort2::merge_sort as merge_sort2;
pub use self::shake_sort::shake_sort;
pub use self::gnome_sort::gnome_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;

#[allow(dead_code)]
fn is_sorted<T: Ord>(arr: &[T]) {
    for i in 0..arr.len()-1 {
        assert!(arr[i] <= arr[i+1])
    }
}