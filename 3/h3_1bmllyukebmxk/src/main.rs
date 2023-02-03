fn main() {
    println!("Hello, world!");
    // 行注释
    /*.............块注释 */

    // 3.1 变量与可变量


    // 变量不可变：let 声明的是变量、但这个变量被绑定后是不可变得
    let _x: i32 = 5;
    println!("变量 let _x = {}",_x);
    
    /*
    _x = 6 // 如果对_x 重新绑定新的值会报错
    println!("变量 let _x = {}",_x);
     */


    // 变量可以改变： 关键字 mut
    let mut _y: i32 = 5;
    println!("可变变量 let mut _y = {}", _y);
    _y = 6;
    println!("重新赋值的可变变量 let mut _y = {}", _y);

    /// 常量关键字 const 任何作用域或者全局声明
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("常量 THREE_HOURS_IN_SECONDS 3小时等于{}秒", THREE_HOURS_IN_SECONDS);


    // 隐藏 遮蔽
    let a: i32 = 5;
    // 在这里a遮蔽了a
    let a: i32 = a + 1;

    {
        let a: i32 = a * 2;
        println!("a在这个作用域的值{}",a);
    }

    println!("a在最外层的值是{}",a);


    // 隐藏遮蔽与mut的区别
    let spaces = "  ";
    println!("spaces未隐藏前{}.",spaces);
    let spaces = spaces.len();
    println!("spaces隐藏后{}",spaces);

    let mut spaces_m = "    ";
    println!("spaces_m{}.", spaces_m);
    // spaces_m = spaces_m.len(); 
    println!("spaces_m{}.",spaces_m)

    /*
        总结：本节学习了rust几个关键字和名词，let：不可变变量，mut：可变变量，常量：const，还有Shadowing（遮蔽或者隐藏）。
    */

}
