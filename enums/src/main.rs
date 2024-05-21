enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum IpAddr1 {
    V4(u8, u8, u8, u8),
    V6(String),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let value = String::from("hello");
    print_hello(value);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback_enum = IpAddrEnum::V6(String::from("::1"));

    let home_enum1 = IpAddr1::V4(127, 0, 0, 1);

    let loopback_enum1 = IpAddr1::V6(String::from("::1"));
}

fn print_hello(value: String) {
    if 1 == 1 {
        println!("{}", value);
    }
}

/*
TODO: metodo de utilização de dados dentro de um enum abstraindo estruturas

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
 */
