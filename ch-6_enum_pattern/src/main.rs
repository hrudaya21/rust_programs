enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6;

    let x: u32 = 5;
    let y: Option<u32> = None;
    let z: Option<u32> = Some(7);
    let sum1 = x + y.unwrap_or(0);
    let sum2: u32 = x + z.unwrap_or(0);
    println!("Sum: {}", sum1);
    println!("Sum: {}", sum2);



}
