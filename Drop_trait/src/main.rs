/* 
 *  Drop trait
 */

struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer {
    // 析构函数
    fn drop(&mut self){
        println!("执行drop方法之后 data = `{}`!", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer{
        data: String::from("hello"),
    };
    {
        let x: CustomSmartPointer = CustomSmartPointer{
            data: String::from("内部的数据"),
        };
        println!("执行drop方法之前 内部的data = `{}`!", x.data);
    }

    // c.drop(); // Error:不能手动调用，这样会被调用两次内存释放，因为系统还会隐式的调用drop(x)方法
    println!("main中的data = `{}`!", c.data);

    let d = CustomSmartPointer{
        data: String::from("Rust"),
    };
    drop(d); // 手动调用
    
    // c在最后被Drop
}
