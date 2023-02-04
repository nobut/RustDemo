struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}
// 类单元结构体
struct AlwaysEqual;
fn main() {
    /*
    结构体和我们在“元组类型”部分论过的元组类似，它们都包含多个相关的值。
    和元组一样，结构体的每一部分可以是不同类型。
    但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。
    由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。
     */

    // 结构体的定义就像一个类型的通用模板，而实例则会在这个模板中放入特定数据来创建这个类型的值
    // 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变
    let mut user1 = User {
        active: true,
        username: String::from("lucien"),
        email: String::from("abc@email.com"),
        sing_in_count: 001,
    };
    println!("user = {}", user1.active);
    println!("user = {}", user1.username);
    println!("user = {}", user1.email);
    println!("user = {}", user1.sing_in_count);
    user1.username = String::from("tony");
    println!("change user name = {}", user1.username);

    let user2: User = build_user(String::from("body"), String::from("cbd@email.com"));
    println!("user2 = {}", user2.username);

    // 结构体更新语法
    let user3 = User {
        username: String::from("siri"),
        ..user1 // 使用了user1实例里面的除了username字段的值进行初始化。
    };

    // println!("user1 = {}",user1.email); // 这里不用在使用user1的email了，
    // 因为user1的email在user3中被move了，但是active喝sing_in_count是被copy的，他们数据基本数据类型。
    println!("user1 = {}", user1.username);
    println!("user3.username = {}", user3.username);

    /*
    也可以定义与元组（在第三章讨论过）类似的结构体，称为 元组结构体（tuple structs）。元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
     */

    struct Clolor(i32, i32, i32);
    struct Point(i32, i32, i32);

    //  注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。
    // 你定义的每一个结构体有其自己的类型，即使结构体中的字段可能有着相同的类型
    let color1 = Clolor(2, 3, 4);
    let point1 = Point(4, 5, 6);

    println!("color1 = {}", color1.0);
    println!("point1 = {}", point1.0);

    /*
    我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）
    因为它们类似于 ()，即“元组类型”一节中提到的 unit 类型。
    类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
     */

    let _subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 001,
    }
}
