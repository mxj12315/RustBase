use std::{rc::Rc, cell::RefCell};

/* 
 * Rc指针和RefCell指针结合，
 *  Rc指针能让一个数据拥有多个不可以变的引用的所有权
 *  RefCell指针能够让一个外部不可以变的引用的内部可变
 */
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::{Cons,Nil};

fn main() {
    let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));// List::Cons()的第一个参数
    let a: Rc<List>= Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));// List::Cons()的第二个参数

    let b: List = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)); // 将a的所有权分配给b，但是a变量不会被销毁
    let c: List = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));// 将a的所有权分配给c，但是a变量不会被销毁

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}
