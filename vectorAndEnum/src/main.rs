// 使用枚举配合数组泪确定数组中的值
enum cell {
    Int(i32),
    Float(f64),
    Text(String),
}
   

fn main() {
    let v = vec![
        cell::Int(3),
        cell::Float(3.14),
        cell::Text(String::from("hello")),
    ];
}
