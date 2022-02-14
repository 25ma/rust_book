
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    println!("hello,unrecoverable errors with result.");

    // result 是一个枚举类型的，它定义有如下两个成员 Ok 和 Err

    // let f = File::open("./hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // println!("f: {:?}", f);


    // 匹配不同的错误

    let f2 = File::open("hello.txt");

    let f2 = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {:?}", e),
            }, 

            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("f2: {:?}", f2);

}