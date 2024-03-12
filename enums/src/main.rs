#[derive(Debug)]
enum IpAddrKind {
    V4(u32, u32, u32, u32),
    _V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let _absent: Option<i32> = None;

    let home = IpAddr {
        kind: IpAddrKind::V4(192, 168, 0, 1),
        address: String::from("127.0.0.1"),
    };

    route(&home);
}

fn route(ip: &IpAddr) {
    println!("kind {:?}", ip.kind);
    println!("address {}", ip.address);
}
