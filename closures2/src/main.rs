use std::thread;

// 闭包通过三种方式捕获其环境
// 不可变借用，
// 可变借用
// 和获取所有权
fn main() {
    // 闭包借用
    let arr = vec![1, 2, 3];
    println!("定义的arr = {:?}", arr);
    // 使用闭包获取不可变借用，借用不会导致所有权的转移
    // 闭包可以直接获取外部变量
    let only_borrows = || {
        println!("闭包内 = {:?}", arr);
    };
    println!("调用闭包之前arr = {:?}", arr);
    only_borrows();
    println!("调用闭包之后arr = {:?}", arr);

    // 闭包修改可变借用
    let mut arr2 = vec![1, 2, 3];
    println!("定义的arr2 = {:?}", arr2);
    // 定义闭包
    let mut borrows = || {
        arr2.push(4);
    };
    // 当 borrows_mutably 定义时，它捕获了 list 的可变引用。
    // 闭包在被调用后就不再被使用，这时可变借用结束 arr2变为不可以变的借用
    // println!("调用闭包之前arr2 = {:?}", arr2); // 不能同时存在可变和不可变引用，此处引用不变引用会报错
    borrows();// 可变的借用
    println!("调用闭包之后arr2 = {:?}", arr2);

    // 使用 move 来强制闭包为线程获取 list 的所有权
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
  
    // 使用move将list变量的所有权转移到新的线程中
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();


}
