pub fn binary_search(haystack: Vec<i32>, needle: i32) -> bool {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let v = haystack[m];

        println!("lo = {lo}; mid = {m}; hi = {hi}");

        if v == needle {
            println!("{needle} is contained in the array!");
            return true;
        } if v > needle {
            hi = m;
        } else {
            lo = m + 1
        }
    }

    println!("{needle} is NOT contained in the array!");
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test_one() {
        let haystack = vec![1, 3, 5, 7, 9];
        let needle = 3;

        assert_eq!(true, binary_search(haystack, needle));
    }

    #[test]
    fn search_test_two() {
        let haystack = vec![1, 3, 5, 7, 9];
        let needle = 8;

        assert_eq!(false, binary_search(haystack, needle));
    }
}
