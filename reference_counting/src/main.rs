/* 
 * Rc<T>指针 reference counting
 * 记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没
 * 有任何有效引用并可以被清理。
 * 
 * Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候
 * Rc<T> 只能用于单线程场景；
 * Rc<T> 指针需要自己导入
 */
// 使用RC智能指针
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a: Rc<List>= Rc::new(Cons(10,Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 打印强引用计数 1
    // 让b和c同时指向a的一部分
    let b: List = Cons(20,Rc::clone(&a)); // 也可以使用a.clone()
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let d = Rc::clone(&a);
        println!("count after creating d = {}", Rc::strong_count(&a)); // 3
    }
    let c: List = Cons(30,Rc::clone(&a));  
    println!("count after creating c = {}", Rc::strong_count(&a)); // 3



}
