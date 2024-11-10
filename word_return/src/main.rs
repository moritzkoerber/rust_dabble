fn main() {
    let s: String = String::from("This string has five words");

    let word: &str = first_word(&s);

    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
