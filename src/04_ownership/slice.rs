
fn main() {
    println!("hello, slice.");

    // slice 允许你引用集合中的一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以他没有所有权。
    
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("word = {}", word);

    s.clear();

    println!("s = {}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}