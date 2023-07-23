/*
Rust中字符串默认是以utf8的编码方式存放
String类型是可变的,可以转移所有权，&str是不可变的，借用
 */
fn main() {
    let mut str: &str = "hello Rust";
    let str_content: String = str.to_string();

    let mut str1 = String::from("world");
    str1.push_str("nihao");
    str1.push('Z');
    println!("str1={}", str1);

    let str2 = String::from("beiJing");
    let str3 = String::from(" shangHai");
    let str4 = str2 + &str3;  // + 相当于执行了 fn add(self, s: &str) -> String方法，str3的所有权转移到了add方法中
    println!("str4={}", str4);
    // println!("str2={}", str2); // 报错 str2的所有权转移到了str4,所以str2不可用
    println!("str3={}", str3); // str3的所有权没有转移

    // 格式化
    let str5 = String::from("1990");
    let str6 = String::from("10");
    let str7 = String::from("12");
    let str8= format!("{str5}-{str6}-{str7}");
    println!("str8:{}",str8);


    // 字符串的截取
    let name = "abdcef";
    let name_slice =  &name[2..=3];
    println!("name_slice={}",name_slice);//dc

    

}
