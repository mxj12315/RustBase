pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messager: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max: f64 = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        sent_messages: RefCell<Vec<String>>, // 定义一个结构体,用于存储发送的消息
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl  Messenger for MockMessenger {
        // send()方法内部会向vec数组中添加一个元素,但是其参数是一个借用
        // 外部值(self)被认为是不可变的情况下修改内部值(self.sent_messages)
        fn send(&self, msg: &str) {
            let mut one_borrow: std::cell::RefMut<Vec<String>> =  self.sent_messages.borrow_mut(); // 第一个可变借用
            // one_borrow 和 two_borrow 都是同一个值的可变借用，不符合同一个作用域中一个变量只能有一个可变借用或者无数个不可变借用
            // let mut two_borrow: std::cell::RefMut<Vec<String>> =  self.sent_messages.borrow_mut(); // 第二个可变借用
            one_borrow.push(String::from(msg));
            // two_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger: MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80); // set_value方法的第一个参数类型&mut self,要求是一个可变的实例
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // 计算mock_messenger.sent_messages内vec数组的长度
    }   
    


}