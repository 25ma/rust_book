
fn main() {
    println!("hello, match.");

    // match 是rust 里极为强大的控制流运算符（模式匹配），它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。
    // 模式可由字面值、变量、通配符和许多其他内容构成


    // 使用match 实现硬币分类器的功能

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin:Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10, 
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }

    println!("value_in_cents :{}", value_in_cents(Coin::Penny));
    println!("value_in_cents :{}", value_in_cents(Coin::Nickel));
    println!("value_in_cents :{}", value_in_cents(Coin::Dime));

    println!("value_in_cents :{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("value_in_cents :{}", value_in_cents(Coin::Quarter(UsState::Alaska)));


    // 匹配 Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //  操作数组
    let mut arr = Vec::new();
    arr.push(six);
    
    arr.push(Some(5));
    arr.push(none);

    // 遍历数组
    for element in arr {

        // 模式匹配
        match element {
            Some(i) => {
                println!("i = {}", i);
            
            },
            None => {
                println!("is none .");
            },
        }
    }


    // 通配模式和_占位模式

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}

    fn remove_fancy_hat() {}

    fn move_player(num_spaces :u8) {
        println!("num = {}", num_spaces);
    }
}