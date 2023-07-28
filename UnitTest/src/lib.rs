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

// 两个数相加
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// 通过断言传入参数
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// 定义tests模块
#[cfg(test)]
mod tests {
    //use super::*;
    // 也可以使用下面显示的引入
    use super::add_two;
    use crate::Rectangle;
    use super::greeting;

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

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let res = greeting("Test参数");
        // 下面使用了断言参数，可以打印提示信息
        assert!(res.contains("Ted"),"这是测试失败的提示信息:res =「{}」",res)
    }
}
