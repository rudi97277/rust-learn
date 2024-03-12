#[derive(Debug)]
enum IpAddrKind {
    V4(u32, u32, u32, u32),
    _V6(String),
}

fn main() {
    let absent: Option<i32> = None;

    let home = IpAddrKind::V4(192, 168, 2, 1);

    println!("value {:?}", home);
}
