
use std::fs::File;
fn main() {
    println!("hello,unrecoverable errors with result.");

    // result 是一个枚举类型的，它定义有如下两个成员 Ok 和 Err

    let f = File::open("./hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("f: {:?}", f);
}