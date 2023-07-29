pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "猜值必须大于1,传入的变量是= {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "猜测传入值要小于100,传入的值是={}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试大于100
    // 该函数未通过测试，正确的发生的了panic，但是预期的信息和panic不符合
    // 打印结果：test tests::greater_than_100 - should panic ... FAILED
    #[test]
    #[should_panic(expected = "这才是panic预期的文字")] // 预期的信息信息
    fn greater_than_100() {
        Guess::new(120);
    }


   // 测试小于1
    // 该函数未通过测试，正确的发生的了panic，panic中的信息包含了should_panic的文字内容
    // 打印结果：test tests::greater_less_1 - should panic ... ok
    #[test]
    #[should_panic(expected = "猜值必须大于1")] // 预期的信息信息
    fn greater_less_1() {
        Guess::new(-1);
    }

    // 传入符合条件的参数
    // 打印结果：est tests::meet_the_conditions ... ok
    #[test]
    fn meet_the_conditions() {
        Guess::new(50);
    }


    // 忽略测试
    // 传入符合条件的参数
    // 打印结果：est tests::meet_the_conditions ... ok
    #[test]
    #[ignore]
    fn meet_the_conditions1() {
        Guess::new(50);
    }

}
