fn main() {
    println!("Hello, world!");

    let s = String::from("Hello, Rust!");
    println!("s = {}", s.len());
    
    for i in 0..s.len() {
        println!("i = {}, char = {}", i, s.chars().nth(i).unwrap());
    }
    for i in 0..10 {
        println!("i = {}", i);
    }

    let mut numbers = vec![9, 8, 7, 10, 4];
    numbers.sort_by(|a, b|a.cmp(b));
    println!("{:?}", numbers);
    print!("{}\n", sum_round(1.9, 2.9));

    let ch: char = 'a';

    println!("Number: {}", ch as i32);
}


fn sum_round (a: f32, b: f32) -> i32 {
    let sum: f32 = a + b;
    return sum as i32;
}