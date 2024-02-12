
pub fn linear_search(haystack: Vec<i32>, needle: i32) -> bool {
    let notpoopy = haystack.len() - 1;

    for n in 0..= notpoopy {
        if haystack[n] == needle {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let haystack = vec![1, 2, 3, 4, 69];
        let needle: i32 = 69;

        assert_eq!(true, linear_search(haystack, needle));
    }

    #[test]
    fn test_two() {
        let haystack = vec![1, 2, 420, 42, 69];
        let needle: i32 = 420;

        assert_eq!(true, linear_search(haystack, needle));
    }

    #[test]
    fn test_three() {
        let haystack = vec![1, 2, 420, 42, 69];
        let needle: i32 = 421;

        assert_eq!(false, linear_search(haystack, needle));
    }
}
