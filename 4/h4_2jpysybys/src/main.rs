fn main() {
    // 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。
    // 与指针不同，引用确保指向某个特定类型的有效值。
    // & 符号就是 引用

    let s = String::from("hello");

    let len = calculate_length(&s);
    println!("the length of '{s}' is {len}。");
    // 当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

    // &mut 可变引用
    let mut ss = String::from("hello");
    change(&mut ss);

    {
        let ss3 = &mut ss;
        println!("这里ss3 = {ss3},可以借用,因为在另一个作用域里面了");
    } // ss3 已经离开了作用域，所以可以有下面新的引用。

    let ss1 = &mut ss;
    // let ss2 = &mut ss;

    println!("ss1 = {ss1}, ss2 = &mut ss不能同时借给两个人");

    /*
    ❌ 数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
    两个或更多指针同时访问同一数据。
    至少有一个指针被用来写入数据。
    没有同步数据访问的机制。*/

    let mut sss = String::from("hello");

    let r1 = &sss;
    let r2 = &sss;
    // let r3 = &mut sss;
    println!("r1 = {r1}, r2 = {r2}, r3 = 之前不可变引用和可变引用不能一起用");

    // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    let r3 = &mut sss;
    println!("r1,r2作用域已经结束了,r3可以用了 = {r3}");

    // 悬垂引用（Dangling References）
    dangle();
    /*
    在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    引用必须总是有效的 */
}

fn calculate_length(s: &String) -> usize {
    // s 是 String的引用。
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", word");
}
fn dangle() -> String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新的字符串
    // Warning
    // &s // 返回字符串s的引用
    s
} // 这里s会被丢弃、内存会释放，很危险。
