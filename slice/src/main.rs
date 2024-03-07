fn main() {
    let mut s = String::from("Hallo world");
    let data = first_word(&s);
    s = String::from("new");
    println!("len : {data}");


}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
