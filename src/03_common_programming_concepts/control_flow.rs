

fn main() {
    println!("hello, control flow.");
    // 控制流 主要分为 if 表达式 和 循环

    // if  else 

    let number = 7;

    if number < 5 && number >1 {
        println!("condition was true");
    } else {
        println!("condition was fasle");
    }
    // 循环
    // loop 循环
    // loop { 
    //     println!("loop again!");   
    // }
    // 这里会是一个无线循环

    // 可以指定标签 类似如下：

    let mut count = 0;
    'counting_up: loop {
        println!("count={}", count);

        let mut remaining = 10;

        // 增加了一个判断避免无线循环
        if count > 20 {
            println!("Count is already greater than 20");
            break;
        }
        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if remaining == 2 {
                break 'counting_up;
            }

            remaining -= 1;
           
        }

        count += 1;
    }

    println!("End count = {}", count);

    // break 后面还可以增加一个表达式

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);

}