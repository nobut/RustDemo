fn main() {
    let config_mac = Some(3u8);
    match config_mac {
        Some(max) => println!("max = {}", max),
        _ => (),
    }
    // if let 是简单的 match 变得简单。
    if let Some(max) = config_mac {
        println!("max = {}", max);
    }
}
