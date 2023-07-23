/*
    方法method
        方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文），并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl中的函数称为关联函数
impl Rectangle {
    // 该函数不是方法，参数中不含有&self
    fn square(wsize:u32,hsize: u32) ->Self{
        Self { width:wsize, height:hsize}
    }
    fn square1(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }


    fn area(&self) -> u32 {
        println!(
            "width={},height={},area={}",
            self.width,
            self.height,
            self.width * self.height
        );
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 5
    }

    /*
       比较矩形
       宽度小于本身并且高度大于本身返回true
    */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && other.height > self.height
    }
}
fn main() {
    // 创建一个矩形A:20*30
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    rect.area();

    println!("width={}", rect.width()); // 调用方法 getters 自动引用和解引用
    println!("width={}", rect.width); // 调用字段

    // 创建一个B:10*40的矩形
    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };
    // 创建一个C:30*40的矩形
    let rect2: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };
    // 比较矩形A和矩形B
    let result1 =  rect.can_hold(&rect1);
    println!("矩形A和矩形B的比较结果:{}",result1); // true
    let result2 =  rect.can_hold(&rect2);
    println!("矩形A和矩形C的比较结果:{}",result2); // false


    // 调用impl中的函数，非方法
    let rect3 =  Rectangle::square(35,46);
    dbg!(rect3);
}
