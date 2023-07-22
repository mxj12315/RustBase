/*
    方法method
        方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文），并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
*/
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!(
            "width={},height={},area={}",
            self.width,
            self.height,
            self.width * self.height
        );
        self.width * self.height
    }
    let a = 2;
    
}
fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    rect.area();
}
