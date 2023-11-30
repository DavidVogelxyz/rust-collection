use std::thread;

fn main() {
    //let mut list = vec![1, 2, 3];
    //println!("Before defining closure: {:?}", list);

    //let only_borrows = || println!("from closure: {:?}", list);

    //println!("Before calling closure: {:?}", list);
    //only_borrows();
    //println!("After calling closure: {:?}", list);

    //let mut list = vec![1, 2, 3];
    //println!("Before defining closure: {:?}", list);

    //let mut borrows_mut = || list.push(7);

    //borrows_mut();
    //println!("after calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("before define closure: {:?}", list);

    thread::spawn(move || println!("from thread: {:?}", list))
        .join()
        .unwrap();
}
