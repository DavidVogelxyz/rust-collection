fn main() {
    use std::collections::HashSet;

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();

        for &n in nums.iter() {

            if map.contains(&n) {
                return true;
            }

            map.insert(n);
        };

        false
    }

    let vec = vec![1, 2, 1, 4];

    println!("{}", contains_duplicate(vec))
}
