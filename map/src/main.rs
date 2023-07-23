/*
    map
 */
use std::collections::HashMap;
fn main() {
    let mut hm = HashMap::new();
    hm.insert("张三", 20);
    hm.insert("李四", 25);
    println!("hm={:?}",hm);

    // 获取map集合的值
    let res = hm.get("张三");
    println!("res={:?}",res);

    // 遍历map集合,item是一个元组
    for item in hm.iter() {
        println!("item={:?}",item);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name 和 field_value 所有权会被移动进入 `map`
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误
    // println!("field_name={:?}",field_name);//value borrowed here after move  所有权被移动了
    // println!("field_value={:?}",field_value);

    // 使用entry方法来插入值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 如果key存在则不插入，如果不存在则插入
    scores.entry(String::from("Blue")).or_insert(50); // 不插入
    scores.entry(String::from("Red")).or_insert(50);// 插入
    println!("scores={:?}",scores);//{"Blue": 10, "Yellow": 50, "Red": 50}
}
