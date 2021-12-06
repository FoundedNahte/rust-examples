enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    }

}

fn ex1(ip_kind: IpAddrKind) {} //takes any kind of IpAddrKind



//more concise way of above
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String:;from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));


// enum variants can have different types and amounts of associated data

enum IpAddr {
    V55(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

// we can put anything inside enums
struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//example

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call()

// Rust doesnt have null but rather 
// enum Option<T> { // the <T> is a generic
//     None,
//     Some(T),
// }

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
