use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number (1-100)");

    let random_value = make_random_value();

    loop {
        let guess = read_guess();
        print_guess(guess);

        match guess.cmp(&random_value) {
            Ordering::Less => {
                println!("Too small");
            }
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => {
                println!("Too large");
            }
        }
    }
}

fn make_random_value() -> u8 {
    return rand::thread_rng().gen_range(1..101);
}

fn read_guess() -> u8 {
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Could not read from stdin");


    return str.trim().parse().expect("Invalid number");
}

fn print_guess(guess: u8) {
    println!("Your guess was {}", guess);
}
