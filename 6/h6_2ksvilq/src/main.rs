#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    /*
    Rust 有一个叫做 match 的极为强大的控制流运算符，它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码
     */

    let coin = value_in_cents(Coin::Quarter((UsState::Alaska)));
    println!("Penny = {}", coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five = {:#?},six = {:#?},none = {:#?}", five, six, none);
}

fn value_in_cents(coin: Coin) -> u8 {
    // match这看起来非常像 if 所使用的条件表达式，不过这里有一个非常大的区别：
    // 对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型的
    // match 分支，一个分支有两个部分：一个模式和一些代码，模式Coin::Penny，代码(表达式) 1.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 0, // 匹配其他模式
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
