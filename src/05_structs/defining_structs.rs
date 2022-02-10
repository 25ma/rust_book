
fn main() {
    println!("hello,structs.");

    // struct 是一个自定义数据结构，允许你包装和命名多个相关的值，从而形成一个有意义的组合。如果你熟悉一门面向对象语言，struct 就像对象中的属性。

    // 定义struct 

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }


    // 创建User 结构体的实例

    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count:1,
    };

    // 打印struct 
    println!("user1 :{:?}", user1);

    let user2 = User{
        email: String::from("25mateam@gmail.com"),
        username: String::from("25ma"),
        ..user1
    };

    println!("user2:{:?}", user2);

    println!("user1 :{:?}", user1);

}