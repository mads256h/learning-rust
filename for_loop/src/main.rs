fn main() {
    let array = ["Hello", "World"];

    for w in array.iter() {
        println!("{}", w);
    }

    for w in array.iter().rev() {
        println!("{}", w);
    }

    for num in 1..4 {
        println!("{}", num);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }
}
