/*
   不使用泛型的情况
*/
// 比较i32类型数组中的最大值
fn largest_i32(list: &[i32]) -> i32 {
    let mut max = list[0];
    for i in list {
        if *i > max {
            max = *i;
        }
    }
    max
}
// 比较char类型数组中的最大值
fn largest_char(list: &[char]) -> char {
    let mut max = list[0];
    for i in list {
        if *i > max {
            max = *i;
        }
    }
    max
}

fn main() {
    // 比较i32类型数组中的最大值
    let number_list = vec![34, 50, 25, 100, 65]; // 新建数组
    let largest = largest_i32(&number_list);
    println!("i32数组largest={}", largest);
    
    // 比较char类型的数组最大字符
    let ch_list = vec!['a','c','S','Z'];    
    let largest = largest_char(&ch_list);
    println!("char数组largest={}",largest);

}
