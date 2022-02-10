
struct Rectangle {
    width: u32,
    height: u32,
}

// 为结构体实现方法
impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }
}

fn main() {
    println!("hello, method syntax.");


    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());
}