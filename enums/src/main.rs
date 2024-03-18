#[derive(Debug)]
enum IpAddrKind {
    V4(u32, u32, u32, u32),
    _V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit(i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let _absent: Option<i32> = None;

    let home = IpAddr {
        kind: IpAddrKind::V4(192, 168, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let new = Message::Quit(32, 50);
    new.call();

    route(&home);
}

fn route(ip: &IpAddr) {
    println!("kind {:?}", ip.kind);
    println!("address {}", ip.address);
}
