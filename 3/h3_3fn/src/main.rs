fn main() {
    /*
    Rust 函数使用关键字：fn声明，main也是一个函数。
    Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词.
     */
    another_function(8);

    /*
        语句和表达式
        语句（Statements）是执行一些操作但不返回值的指令。 
        表达式（Expressions）计算并产生一个值。
     */

     // 这是一个语句有";"
     let x: i32 = 6;

     let y: i32 = {
        let a: i32 = 1;
        a + 1
     };
    //  这是一个表达式、因为x + 1最后没有; 返回了x + 1（有计算），所以是表达式。
    //  {
    //     let a: i32 = 1;
    //     a + 1
    //  }

     println!("y的值是{}",y);

     println!("有返回值的函数add_five(1)(参数是1) = {}",add_five(1));
}

fn another_function(x: i32) {
    println!("调用了another_function,参数是{x},没有返回值。");
}

fn add_five(x: i32) -> i32 {
    x + 5
}