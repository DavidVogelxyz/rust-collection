use std::cmp::Ordering;

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let m = l + (r - l) / 2;

        match target.cmp(&nums[m]) {
            Ordering::Equal => return m as i32,
            Ordering::Less => r = m,
            Ordering::Greater => l = m + 1,
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_not_in_list() {
        let nums = vec![1, 11, 13, 22, 36, 39];
        let target = 229;

        assert_eq!(-1, binary_search(nums, target));
    }

    #[test]
    fn position_zero() {
        let nums = vec![1, 11, 13, 22, 36, 39];
        let target = 1;

        assert_eq!(0, binary_search(nums, target));
    }

    #[test]
    fn position_one() {
        let nums = vec![1, 11, 13, 22, 36, 39];
        let target = 11;

        assert_eq!(1, binary_search(nums, target));
    }


    #[test]
    fn position_three() {
        let nums = vec![1, 11, 13, 22, 36, 39];
        let target = 22;

        assert_eq!(3, binary_search(nums, target));
    }

    #[test]
    fn position_five() {
        let nums = vec![1, 11, 13, 22, 36, 39];
        let target = 39;

        assert_eq!(5, binary_search(nums, target));
    }
}
