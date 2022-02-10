
fn main() {
    println!("hello, defining enums.");

    // 枚举是一个不同于结构体的定义自定义数据类型的方式。

    // 定义enum 

    #[derive(Debug)]
    enum IpAddrKind{
        V4,
        V6,
    }

    // 定义ip地址结构体
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home ip address ={:?}", home);
    println!("loopback ip address ={:?}", loopback);


    // Option 枚举
    // Option 是标准库定义的另一个枚举，Option 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。

    let some_number = Some(3);

    let some_string = Some("a string.");

    let absent_number: Option<i32> = None;


    assert_eq!(some_number, Some(3));

    assert_eq!(some_string, Some("a string."));

    assert_eq!(absent_number, None);
}
