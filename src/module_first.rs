pub fn print_characters() {
    for c in (b'Z'..=b'a').rev() {
        if c.is_ascii_alphabetic() {
            println!("{}", c as char);
        }
    }
}
