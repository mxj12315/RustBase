// 定义test模块
#[cfg(test)] // 预处理
mod test {
    #[test]
    fn add() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

// 定义结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 结构体Rectangle的实现
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// 定义tests模块
#[cfg(test)]
mod tests {
    //use super::*; 
    // 也可以使用下面显示的引入
    use crate::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}