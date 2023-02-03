fn main() {
    // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。

    // let mut s =
    let s: String = String::from("hello world");

    let word: &str = first_word(&s);

    // s.clear();
    println!("the first word is: {word}");

    // 这里 s 的类型是 &str：
    // 它是一个指向二进制程序特定位置的 slice。
    // 这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。
    let _s: &str = "hello word"; // "hello word" 是字符串字面量。

    let my_string: String = String::from("hello word");

    // 'first_word' 适用于 'String' (的 slice)，整体或者全部。
    let word: &str = first_word(&my_string[0..6]);
    println!("word = {word}");
    let word: &str = first_word(&my_string[..]);
    println!("word = {word}");
    // 'first_word' 也适用于 'String'的引用，
    // 这等价于 'String' 的 slice
    let word: &str = first_word(&my_string);
    println!("word = {word}");

    let my_string_literal: &str = "hello word";

    // 'first_word' 适用于字符串字面值，整体或者全部
    let word: &str = first_word(&my_string_literal[0..6]);
    println!("word = {word}");
    let word: &str = first_word(&my_string_literal[..]);
    println!("word = {word}");

    // 因为字符串字面值已经 **是** 字符串 slice了，
    // 这也是适用的，无需 slice 语法！
    let word: &str = first_word(my_string_literal);
    println!("word = {word}");
}

fn first_word(s: &str) -> &str {
    // 这个函数的参数是 &str(他是一个内存地址)
    // fn first_word(s: &String) -> &str { // $String 这个是引用了String
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
