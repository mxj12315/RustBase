/*
    综合小案例，计算矩形面积
*/
  // 使用结构体
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
fn main() {
    // 使用函数
    let width1 = 30;
    let height1 = 50;

    println!(
        "矩形面积是：{} 平方像素",
        area(width1, height1)
    );

    // 使用元组
    let rect1: (u32, u32) = (30, 50);
    println!(
        "矩形面积是：{} 平方像素",
        area1(rect1)
    );


    // 使用结构体
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    area2(&rect2);
    println!(
        "矩形面积是：{:#?} 平方像素",
        area2(&rect2)
    );

    
    // 使用debbug打印结构体
    dbg!(rect2);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

// 使用结构体
fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
    
