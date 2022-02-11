
fn main() {
    println!("hello, hash maps.");

    // hash map 存储键值对

    // hash map 类型储存了一个键类型 k 对应一个值类型的映射，它通过一个哈希函数 来实现映射，决定如何将键和值放入内存中。

    // 新建一个hash map 

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.insert(String::from("Yellow"), 50);

    println!("scores : {:?}", scores);

    for (team,score) in scores {
        
        println!("team: {}, score: {}" ,team, score);
    }


    let teams = vec![String::from("Blue"), String::from("Yellow")];

    let inital_scores= vec![10,50];

    let scores2: HashMap<_,_> = 
        teams.into_iter().zip(inital_scores.into_iter()).collect();

    for (team , score) in scores2 {
        println!("team: {}, score: {}" ,team, score);
    }

    // hashmap 的所有权
    // 对于像 i32 这样的实现了 Copy trait 的类型， 其值就是可以拷贝进哈希map, 对于像string 这样的拥有所有权的值，其值将被移动而哈希map 会成为这些值的所有者

    let field_name = String::from("Favorite color");

    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 此时 field_name 和 field_value 不在有效，因为其所有权都move 给了 哈希map。
    //println!("field_name : {}", field_name);

    // 更新哈希 map 

      // 覆盖一个值

      let mut s2 = HashMap::new();
      s2.insert(String::from("Blue"), 10);
      s2.insert(String::from("Blue"), 25);

      println!("s2: {:?}", s2);

      // 只在健没有对应值时插入

      let mut s3 = HashMap::new();

      s3.insert(String::from("Blue"), 10);
      s3.entry(String::from("Yellow")).or_insert(50);
      s3.entry(String::from("Blue")).or_insert( 50);

      println!("s3 {:?}", s3);

     
      //  根据旧值更新一个值

      let text = "hello world wonderful world";

      let mut m1 = HashMap::new();

      for word in text.split_whitespace() {
          let count = m1.entry(word).or_insert(0);
          *count += 1;
      }

      println!("m1 : {:?}", m1);
      // 打印出的key ,value 的顺序不是固定的，每次刷新顺序都不相同
}