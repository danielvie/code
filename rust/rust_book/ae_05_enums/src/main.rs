fn main() {

    // defining an Enum
    enum IpAddrKind {
        V4,
        V6,
    }
    
    // enum values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // using in the function
    
    fn route(ip_kind: IpAddrKind) { }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    // we might want to use structs for:
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    // but this can be represented in the enum as
    
    enum EIpAddr {
        V4(String),
        V6(String),
    }

    let home = EIpAddr::V4(String::from("127.0.0.1"));

    let loopback = EIpAddr::V6(String::from("::1"));

    
    // we can even define some more advanded enums
    
    enum EEIpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = EEIpAddr::V4(127, 0, 0, 1);

    let loopback = EEIpAddr::V6(String::from("::1"));
    
    println!("end");
    
}
