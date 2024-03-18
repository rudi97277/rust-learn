fn main() {
    let s = String::from("Hallo world");
    let data = first_word(&s[..]);
    println!("len : {data}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
