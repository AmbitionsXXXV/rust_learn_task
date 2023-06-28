pub fn print_characters() {
  for c in b'A'..=b'z' {
      if c.is_ascii_alphabetic() {
          println!("{}", c as char);
      }
  }
}
