/*
 元组结构体
 类单元结构体
*/
fn main() {
    #[derive(Debug)]
    struct Color(i32, i32, i32); // 元组结构体不需要定义字段名
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black={:?}", black);
    println!("origin={:?}", origin);

    #[derive(Debug)]
    struct UnitStruct; // 类单元结构体
    let unit = UnitStruct;
    println!("unit={:?}", unit);
}
