#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // 在 {} 中加入 :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式。
    // {:#?} 这个样式是打印格式化结构体。
    println!("rect1 is {:#?}", rect1);

    // 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。
    // dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg!
    // 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
    /*
    注意：调用 dbg! 宏会打印到标准错误控制台流（stderr），与 println! 不同，后者会打印到标准输出控制台流（stdout）。
    我们将在第十二章 “将错误信息写入标准错误而不是标准输出” 一节中更多地讨论 stderr 和 stdout。
     */
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: (50),
    };
    dbg!(&rect2); // 这里使用了引用、没有获取rect2的所有权。
                  // dbg!(rect2); // 获取了rect2的所有权，再去打印会报错。
    println!("{:#?}", rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    // 访问对结构体的引用的字段不会移动字段的所有权.
    rectangle.height * rectangle.width
}
