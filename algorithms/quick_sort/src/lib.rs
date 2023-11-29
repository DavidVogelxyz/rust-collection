use rand;

pub fn quick_sort(slice: &mut [i32]) {
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
        assert_sorted(slice);
    }
}

pub fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }

        j += 1;
    }

    slice.swap(i, len - 1);

    i
}

pub fn assert_sorted(slice: &[i32]) {
    for i in 1..slice.len() {
        assert!(slice[i - 1] <= slice[i])
    }
}

pub fn random_vector(size: u32) -> Vec<i32> {
    let mut slice: Vec<i32> = Vec::new();
    for _ in 0..size {
        slice.push(rand::random::<i32>());
    }
    slice
}

pub fn run_benchmark(size: u32) {
    let mut vec = random_vector(size);

    quick_sort(&mut vec);
    assert_sorted(&vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_test() {
        run_benchmark(100);
    }
}
