pub fn bubble_sort(arr: &mut [i32]) {
    let mut count: usize = 0;
    let mut new_len: usize;
    let mut len = arr.len();

    loop {
        new_len = 0;

        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }

        if new_len == 0 {
            break;
        }

        len = new_len;
        count += 1;
        println!("{:?}: {:?}", count, arr);
    }
}
