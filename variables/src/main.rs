fn main() {
    let x = 50_0_0;

    let x = x + 1;

    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: u32 = match "a42".parse() {
        Ok(value) => value,
        Err(_) => 0,
    };

    if true {
        println!("halo");
    }

    let cat: &str = "";

    println!("Value of guess {guess} {cat}");

    println!()
}
