fn main() {
    let s = String::from("hello cat");
    take_owner(&s);

    let s2 = give_ownership();
    println!("{s2}");

    let s3 = String::from("new Cart");
    let s4 = takes_and_give_back(s3);

    println!("{s4}");

    let s5 = String::from("memory cat");
    let (s5, length) = calculate_length(s5);

    println!("string : {s5} with size : {length}");

    let size = calculate_length_using_reference(&s5);

    println!("size {size}");

    let mut s7 = String::from("owner from data");

    try_change_borrowing(&mut s7);

    let set = &mut s7;

    try_change_borrowing(set);

    println!("{} ss", set);
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn try_change_borrowing(s: &mut String) {
    s.push_str("da");
}

fn calculate_length_using_reference(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn take_owner(data: &String) {
    println!("{data}");
}

fn give_ownership() -> String {
    let some_string = String::from("hello new");
    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}
