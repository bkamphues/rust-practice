use std::string::String;

fn main() {
    let s = String::from("sjababing hello world");

    let test = first_word(&s);

    println!("{test}");
}

fn first_word(s: &String) -> &str {
    // Grab the string as bytes.
    let bytes = s.as_bytes();

    // Iterate using slice.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
