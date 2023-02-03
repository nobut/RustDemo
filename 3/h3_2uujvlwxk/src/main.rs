fn main() {
    // Rust 是静态语言、声明变量的时候、一般可以不声明类型，编译器能推断出来，除非特殊情况。
    // Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。


    // 整形数据类型：默认i32、范围是2-128位，其中isize（有符号）根据系统架构决定的、uisize无符号。
    let x: isize = 1;
    println!("isize{}",x);

    // 浮点型：所有浮点型都是有符号的。默认是f64（双精度浮点型），但f32精度高（单精度浮点型）。
    let y: f64 = 2.0;
    println!("默认浮点型是f54位{}",y);
    let z: f32 = 1.0;
    println!("f32位精度高{}",z);


    // 布尔值
    let t: bool = true;
    println!("布尔值{}",t);

    /*
    单引号声明 char 字面量，而与之相反的是，使用双引号声明字符串字面量
    Rust 的 char 类型的大小为四个字节 (four bytes)，
    在 Rust 中，带变音符号的字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
    并代表了一个 Unicode 标量值（Unicode Scalar Value），
    这意味着它可以比 ASCII 表示更多内容。
     */
    let c: char = '你'; // '你好啊'会报错。
    let e: char = '🙄';
    println!("char-{}-{}",c,e);


    // 复合型

    /*
        元组
        不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。
        这种值以及对应的类型都写作 ()，表示空值或空的返回类型。
        如果表达式不返回任何其他值，则会隐式返回单元值。
    */

    let tup: (i32, f64, i32) = (9, 8.0, 66);

    // 解构
    let (_e,_f,_g): (i32, f64, i32) = tup;
    println!("e = {}",_e);

    let nine: i32 = tup.0;
    println!("元组解构nine={}",nine);

    // 数组（array）
    let _array: [i32; 6] = [0,1,2,3,4,5];

    // 创建5个3.
    let _q: [i32; 5] = [3; 5];

    _q.into_iter().for_each(|value: i32| {
        println!("数组array的value = {}",value);
    });

}
