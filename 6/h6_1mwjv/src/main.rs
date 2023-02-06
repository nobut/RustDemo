enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum Ipaddress {
    V4(u8, u8, u8, u8),
    V6(String),
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

    /*
    Option 是标准库定义的另一个枚举。
    Option 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。
     */
    // Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 Option<T>
    let x: i8 = 1;
    let y: Option<i8> = Some(2);
    // let sum = x + y; // 会报错、因为i8和Option<i8>不是一个类型。
    let absent_number: Option<i32> = None;
    /*
    为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 Option<T> 中。
    接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空。
    这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。
     */
}
