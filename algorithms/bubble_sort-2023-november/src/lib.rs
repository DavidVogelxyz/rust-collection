pub fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let mut result: Vec<T> = collection.into();

    for _ in 0..result.len() {
        for i in 1..result.len() {
            if result[i - 1] > result[i] {
                result.swap(i - 1, i);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let nums = vec![5, 4, 3, 2, 1];

        assert_eq!(vec![1, 2, 3, 4, 5], bubble_sort(&nums));
    }

    #[test]
    fn test_bubble_sort() {
        assert_eq!(vec![2, 3, 4, 5], bubble_sort(&vec![5, 4, 3, 2]));
        assert_eq!(vec![2, 3, 3, 5], bubble_sort(&vec![5, 3, 3, 2]));
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(&vec![4, 1, 3, 2]));
    }

    #[test]
    fn list_already_sorted() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(&vec![1, 2, 3, 4]));
    }
}
