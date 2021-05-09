enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

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
    };

    let home_enum_constructor = IpAddrEnum::V4(127, 0, 0, 1);

    let loopback_enum_constructor = IpAddrEnum::V6(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();

    // No nulls, instead we have the option enum
    let some_number = Some(5);

    let some_string = Some("I can become 'none'");

    let absent_number: Option<i32> = None;
}
