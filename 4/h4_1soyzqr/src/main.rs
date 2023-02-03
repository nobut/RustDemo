fn main() {
    /*
        所有权规则
    1.Rust 中的每一个值都有一个 所有者（owner）。
    2.值在任一时刻有且只有一个所有者。
    3.当所有者（变量）离开作用域，这个值将被丢弃。*/

    let _s: () = {
        // x 是无效的，没有声明
        let x: i32 = 1; // x 是有效的
        println!("x = {x}");
    }; // 离开作用域 x 无效的。
       //  println!("x = {x}"); x not found in this scope

    // String

    let mut s: String = String::from("hello");
    s.push_str(", world");
    println!("{s}");
    // 移动 只是移动了引用（也就是栈上的数据）
    let s1: String = String::from("value");
    let s2: String = s1;
    println!("s1的值被移动指向s2了,s1此时无效了,s1是{s2}");
    // 克隆 是真的拷贝、堆上的数据也会被拷贝。
    let s3: String = s2.clone();
    println!("s3的值被克隆到了s3 = {s3},s2 = {s2}也可以用");
    // 只在栈上的数据：拷贝
    /*
        所有整数类型，比如 u32。
    布尔类型，bool，它的值是 true 和 false。
    所有浮点数类型，比如 f64。
    字符类型，char。
    元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。 */
    let x: i32 = 5;
    let y: i32 = x;
    println!("简单数据能copy。x = {x}, y = {y}");

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值被移动到函数里

    // println!("s = {s}"); // s 被移走了不能调用了。

    let x = 5; // x 进入作用域

    makes_copy(x); // x 被copy了。

    println!("原始 x = {x}"); // 因为x被拷贝，所以这里还能用。

    let y = String::from("hello");

    let z = takes_and_gives_back(y);

    println!("z = {z}");

    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。


    // Rust 对此提供了一个不用获取所有权就可以使用值的功能，叫做 引用（references）。
    let s5 = String::from("hello");

    let (s6, len) = calculate_length(s5);
    println!("the length of '{}', is {}。", s6, len);
    
} // 这里，x 现已移除作用域，然后是 s。

fn takes_ownership(some_string: String) {
    // some_string进入作用域。
    println!("被移动过来重新绑定 some_string{some_string}");
} // 这里，some_string 移除作用域并调用 'dorp' 方法。
  // 占用的内存被释放。

fn makes_copy(some_interger: i32) {
    // some_interger 进入作用域
    println!("被拷贝过来的 some_interger = {some_interger}");
} // some_interger 移除作用域。

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串长度
    (s, length)
}
