/*
Rust中字符串默认是以utf8的编码方式存放
 */
fn main() {
    let str: &str = "hello Rust";
    let str_content: String = str.to_string();

    let mut str1 = String::from("world");
    str1.push_str("nihao");
    str1.push('Z');
    println!("str1={}", str1);

    let str2 = "beiJing";
    let str3 = " shangHai";
    let str4 = str2.to_owned() + str3;
    println!("str4={}", str4);

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
