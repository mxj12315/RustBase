pub mod front_of_house {
    use std::fmt::Result;
    pub use std::io::Result as IoResult;
    use std::{cmp::Ordering};
    use std::io::{self, Write};  // 等同 use std::io+std::io::Write;
    use std::collections::*;

    fn function1() -> Result {
        // --snip--、
        Result::Ok(())
    }
    
    fn function2() -> IoResult<()> {
        // --snip--
        IoResult::Ok(())
    }


    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        fn  fix_incorrect_order() {
            super::hosting::add_to_waitlist(); // super相当于 .. 返回上一级菜单
        }
    }

    // 定义一个公共的结构体，但是其内部的字段并不是全部公开的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 同一级访问
    pub fn eat_at_restaurant1() {
        // 在夏天订购一个黑麦土司作为早餐
        let mut meal = Breakfast::summer("Rye");
        // 改变主意更换想要面包的类型
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    
        // 如果取消下一行的注释代码不能编译；
        // seasonal_fruit字段是私有的同一级可以访问
         meal.seasonal_fruit = String::from("blueberries");
    }

    // 枚举
    pub enum Appetizer {
        Soup,
        Salad,
    }

}


pub fn eat_at_restaurant() {
    // 使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist;
    // 使用相对路径
    front_of_house::hosting::add_to_waitlist;

}

// 外部访问私有结构体字段
pub fn eat_at_restaurant2() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal =front_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

// 外部访问私有的枚举字段,不报错
pub fn eat_at_restaurant3() {
    let order1 = front_of_house::Appetizer::Soup;
    let order2 = front_of_house::Appetizer::Salad;
}

