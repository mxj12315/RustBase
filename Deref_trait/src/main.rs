use std::ops::Deref;

/* 
 * Deref_trait 解引用trait，可以使用*解引用
 * Deref 强制转换
 */
// 元组结构体
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self)->&Self::Target {
        &self.0   //.0 用来访问元组结构体的第一个元素
    }

  
}
impl<T> MyBox<T> {
    fn new(t: T)->MyBox<T>{
        MyBox(t)
    }
}
fn hello(name:&str){
    println!("hello {}",name);
}

fn main() {
    let x = 5;
    let y = &x;
    println!("{}", *y);
    assert_eq!(5, *y);
    // Error: no implementation for `{integer} == &{integer}`
    // assert_eq!(5,y);

    // 使用Box<T>来包装
    let z = 5;
    let w = Box::new(z);
    println!("{}", *w);
    assert_eq!(5, *w);


    // 使用自定义的MyBox
    let n= MyBox::new(5);
    // 底层是 *(y.deref())  线雕用deref() 在使用*解引用
    assert_eq!(5, *n);   // 可以对n进行*解引用  


    // 实现Deref trait的结构，可以自动解引用
    let m = MyBox::new(String::from("Rust"));
    // hello方法参数要求是&str，但是传入的m是MyBox<String>类型的引用，
    // 因为Mybox已经实现了Deref trait，底层会自动进行解引用操作
    hello(&m); 
    // 如果没有实现Deref trait，则需要这样
    // 对m:String进行解引用,然后使用[..]切片
    hello(&(*m)[..]);

}
