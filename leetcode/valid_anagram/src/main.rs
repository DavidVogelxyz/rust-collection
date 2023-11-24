use std::collections::HashMap;

fn main() {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, i32> = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *map.entry(a).or_default() += 1;
            *map.entry(b).or_default() -= 1;
        }

        map.into_values().all(|cnt| cnt == 0)
    }

    let s = "999283".to_string();
    let t = "8327".to_string();

    println!("{}", is_anagram(s, t));
}
