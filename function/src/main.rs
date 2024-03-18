fn main() {
    let y = {
        let x = 3;
        x * 2
    };

    let d = if true { 5 } else { 2 };

    println!("Hello, world! {y}");
    another_function("halo");
    let value = plus_one(10);
    println!("data {value}");

    for number in 1..=10 {
        println!("{number}");
    }
}

fn another_function(data: &str) {
    println!("sent data {data}")
}

fn plus_one(x: i32) -> i32 {
    if false {
        return x + 1;
    }

    5
}
