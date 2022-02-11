
fn main() {
    println!("hello,unrecoverable errors with panic.");

    // panic 是rust 的宏， 当执行这个宏时，程序会打印出一些错误信息，展开并清理栈数据，然后接着退出。出现这种情况的场景通常是检测到一些类型的 bug, 而且程序员并不清楚该如何处理它。

    // 在程序中简单的调用panic!;

    // panic!("crash and burn");

    // 使用panic!的backtrace

    let v = vec![1,2,3];

    v[99];
}