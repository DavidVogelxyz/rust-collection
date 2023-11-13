fn main() {
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let truncated = -5 / 3;

    let remainder = 43 % 5;

    println!("the sum is: {sum}");

    println!("the difference is: {difference}");

    println!("the product is: {product}");

    println!("the quotient is: {quotient}");

    println!("the truncated is: {truncated}");

    println!("the remainder is: {remainder}");

    let t = true;

    let f: bool = false;

    println!("t: {t}; f: {f}");

    let c = 'z';

    let z: char = 'Z';

    let heart_eye_cat = '😻';

    println!("{c} {z} {heart_eye_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{x}, {y}, {z}");

    let tupe: (i32, f64, u8) = (500, 6.4, 1);

    let xn = tupe.0;

    let yn = tupe.1;

    let zn = tupe.2;

    println!("{xn}, {yn}, {zn}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //println!("{a}");

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let test = a[3];

    let first = months[0];

    println!("pooooo {test}");

    println!("my options expire in {first}");
}
