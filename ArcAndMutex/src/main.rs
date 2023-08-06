/*
   Arc原子计数指针
   Mutex多个线程间共享值 这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样
*/

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
fn main() {
    // Mutex指针使其内部可变和RefCell一样   
    // Arc指针对多线程中共享变量的所有权进行共享
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter); // 这将创建另一个指向相同分配的指针，增加强引用计数
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            //lock 调用 返回一个叫做 MutexGuard 的智能指针。这个智能指针实现了 Deref 来指向其内部数据
            let mut num: MutexGuard<i32> = counter.lock().unwrap(); // lock()获取一个互斥锁，阻塞当前线程直到它能够获取到互斥锁
            *num += 1;// 智能指针实现了解引用来指向其内部数据
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());// 主线程中的锁
}
