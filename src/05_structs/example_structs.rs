
fn main() {
    println!("hello, example structs.");

    // 指定计算长方形的宽和高，来计算长方形的面积

    // 普通实现

    let width1 = 30;
    let height1 = 50;

    println!("the area of the rectangle id {} square pixels.", area1(width1, height1));


    // 使用元组实现

    let rect2 = (30,50);
    println!("tuple the area of the rectangle id {} square pixels.", area2(rect2));

    // 使用结构体
    #[derive(Debug)]
    pub struct Rectangle{
        width: u32,
        height: u32,
    }



    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("struct the area of the rectangle id {} square pixels.", area3(&rect3));

    
    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

fn area1(width:u32, height:u32) -> u32 {
    width * height
}

fn area2(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}




// 由此得知 struct 是需要放在main 方法之外，或者 实现其他方法

