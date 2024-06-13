enum IpAddr {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}



fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // Both valid as they're now of type IpAddrKind
    route(four);
    route(six);

    // Associate a string with each enum type, avoids needing a struct
    let home = IpAddr::V4(String::From("127.0.0.1")); 
    // Ip addr is already built into rust standard library
}

fn route(ip_kind: IpAddrKind) {

}

// Q-Graph example for enum? perhaps not tbh
enum NextNode {
    Forward(Node),
    Up(Node),
    Down(Node),
    RotateClockwise(Node),
    RotateCounterClockwise(Node),
}