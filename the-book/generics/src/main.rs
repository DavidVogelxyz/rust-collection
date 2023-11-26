fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![24, 50, 35, 100, 69];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![21, 53, 15, 10, 69];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['u', 'x', 'y', 'd', 'w'];

    let result = largest(&char_list);
    println!("The largest number is {}", result);
}
