
fn main () {
    println!("hello, if let.");

    // if let 语法让我们以一种冗长的方式结合 if 和 let, 来处理只匹配一个模式的值而忽略其他模式的情况

    let config_max = Some(3u8);

    // match 的方式
    match config_max {
        Some(max) => println!("the maximum is configured to be {}", max),
        _ => (),
        
    }

    // if let 的方式,是代码更加简洁

    if let Some(max) = config_max {
        println!("the maximum is configured to be {}", max);
    }
}