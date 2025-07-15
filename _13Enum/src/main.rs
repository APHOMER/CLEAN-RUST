//  ENUM: IS A VERSATILE TOOL USED TO REPRESENT A TYPE THAT CAN TAKE ON ONE OF SEVERAL POSSIBLE VARIANTS.

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddeKind = IpAddeKind::V6;

    fn route(_ip_kind: IpAddeKind) {}

    route(_ip_kind: IpAddeKind::V4);
    route(_ip_kind: IpAddeKind::V6);

    // Using Enums 
    // let home: IpAddr = IpAddr::v4(String::from("127.0.0.1"));
    // let loopback: IpAddr = IpAddr::V6(String::from("::1"));



    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // Enhanced Enums
    let home: IpAddr = IpAddr::V4("127,0,0,1");
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
















    // // USING STRUCT
    // enum IpAddrKind {
    //     v4,
    //     v6
    // }

    // struct IpAddr{
    //     kind: IpAddrKind,
    //     address: String
    // }

    // let home: IpAddr = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback: IpAddr = IpAddr{
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

}
