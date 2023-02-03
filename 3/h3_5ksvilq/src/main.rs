fn main() {
    println!("Hello, world!");

    /*
        if 表达式
        其中 if 条件必须是bool类型 { }
     */
    let x = 3;
    if x < 5 {
        println!("x小于5");
    } else if x == 5 {
        println!("x等于5");
    }else {
        println!("x大于5");
    }

    let condition: bool = true;

    /*
        下面的数字5 6都是表达式返回给number、其中如果if后面的块里面“5”是整型、那么else就不能是char或者其他类型。
     */
    let number: i32 = if condition {
        5
    }else {
        6
    };

    println!("nmuber = {}",number);

    /*
        loop循环
     */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }

        println!("loop循环 = {}",counter);
    };
    println!("ruslt = {}",result);

    // 循环标签
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("Edn count = {count}");

    // while 语句

    let mut n: i32 = 3;
    while n != 0 {
        println!("n = {n}");
        n -= 1;
    }

    println!("n = {n}");

    // for 循环
    (1..4).rev().for_each(|value: i32| {
        println!("for {value}");
    });

}
