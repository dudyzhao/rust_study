// #[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let homeEnum = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopbackEnum = IpAddrEnum::V6(String::from("::1"));

    let homeEnumMore = IpAddrMore::V4(127, 0, 0, 1);
    let loopbackMOre = IpAddrMore::V6(String::from("::1"));

    let messageEnum = Message::Write(String::from("hello"));
    messageEnum.call();
}

// 枚举类型
enum IpAddrKind {
    V4,
    V6,
}
// 带参数枚举
enum IpAddrEnum {
    V4(String),
    V6(String),
}
// 带多个参数枚举：每个成员可以处理不同类型和数量的数据
enum IpAddrMore {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Message {
    Quit,                       // 无参
    Move { x: i32, y: i32 },    // 某结构体
    Write(String),              // 单一类型
    ChangeColor(i32, i32, i32), // 多类型
}

impl Message {
    fn call(&self) {
        println!("{}", &self::Write);
    }
}
// fn route(ip_kind: IpAddrKind) {}
