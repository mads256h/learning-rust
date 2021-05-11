use std::io;

fn main() {
    let number = read_input();

    let mut a = 1;
    let mut b = 0;

    for _ in 1..number {
        let w = b;
        b = a;
        a += w;
    }

    println!("Fibonacci: {}", a);
}

fn read_input() -> i32 {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Unable to read stdin");

    return str.trim().parse().expect("Invalid number");
}
