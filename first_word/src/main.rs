fn main() {

    println!("{}", first_word("Hello World!"));
    println!("{}", first_word("Hello-World!"));
}

fn first_word(string: &str) -> &str {
    for (i, &ch) in string.as_bytes().iter().enumerate() {
        if ch == b' ' {
            return &string[0..i];
        }
    }

    return string;
}
