pub fn binary_search_list(haystack: Vec<i32>, needle: i32) -> i32 {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m = lo + ((hi - lo) / 2);
        let v = haystack[m];

        if v == needle {
            return v;
        } if v > needle {
            hi = m;
        } else {
            lo = m +1
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 1;

        assert_eq!(1, binary_search_list(haystack, needle))
    }

    #[test]
    fn test_two() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 2;

        assert_eq!(2, binary_search_list(haystack, needle))
    }

    #[test]
    fn test_three() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 3;

        assert_eq!(3, binary_search_list(haystack, needle))
    }

    #[test]
    fn test_four() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 4;

        assert_eq!(4, binary_search_list(haystack, needle))
    }

    #[test]
    fn test_five() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 5;

        assert_eq!(5, binary_search_list(haystack, needle))
    }
}
