enum IpAddrKind {
    V4,
    V6
}

enum IpAddrKindWithString {
    V4(String),
    V6(String)
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let x = IpAddrKindWithString::V4(String::from("127.0.0.1"));
    // can be done in one line with enums

    let m = Message::Write(String::from("Hello!"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {

}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}
 enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
 }

 impl Message {
    fn call(&self) {
    //do something
    }
 }
