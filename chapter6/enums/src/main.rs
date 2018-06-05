#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    address: String,
    kind: IpAddrKind,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// The following enum is equivalent to the subsequent 4 structs
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But we can define a fonction for all the variants of the enum at once
impl Message {
    fn call(&self) {
        // body
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{} {:#?}", home.address, home.kind);

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home = IpAddr2::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr2::V6(String::from("::1"));

    let _home = IpAddr3::V4(127, 0, 0, 1);
    let _loopback = IpAddr3::V6(String::from("::1"));

    let _qm = QuitMessage {};
    let mm = MoveMessage { x: 1, y: 2 };
    let _wm = WriteMessage(String::from("Hi"));
    let _ccm = ChangeColorMessage(255, 12, 37);
    println!("{} {}", mm.x, mm.y);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("Hi"));
    let m4 = Message::ChangeColor(255, 12, 37);
    m1.call();
    m2.call();
    m3.call();
    m4.call();
}
