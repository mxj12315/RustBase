use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
enum  List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}

use List::{Cons,Nil};
impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match self{
            Cons(_,item) => Some(item),
            Nil => None,
        }
    }
}

fn  main() {
    // 使用Rc指针使得a变量的所有权可以分配给多个变量
    let a: Rc<List>= Rc::new(Cons(6,  RefCell::new(Rc::new(Nil))));
    println!("Rc指针a变量最初计数={}",Rc::strong_count(&a));
    println!("a 的成员={:?}", a.tail());

    let b: Rc<List> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("Rc指针a变量计数={}",Rc::strong_count(&a));// 这里对a的引用计数+1
    println!("Rc指针b变量最初计数={}",Rc::strong_count(&b));
    println!("b 的成员={:?}", b.tail());

    // if let 模式匹配 匹配成功则link被赋值为item
    if let Some(link)  = a.tail(){ 
        *link.borrow_mut() = Rc::clone(&b); // 这里对b的引用计数+1，a的第二项则会引用b
        // 相当于
        
    };
    
    println!("B改变a后的rc计数= {}", Rc::strong_count(&b)); // 2
    println!("a改变a后的rc计数 = {}", Rc::strong_count(&a));// 2


    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());


}
