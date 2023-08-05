use std::{thread, time::Duration};

/* 
Rust的多线程编程
 */
fn main() {
    // 主线程结束则子线程也会一起结束，使用join方法，可以等待子线程结束
   let handler =  thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}

#[cfg(test)]
mod tests{
    use std::thread;
    #[test]
    fn test(){
        let v = vec![1,2,3];
        // 需要使用move关键字将v的所有权转移到新的线程中
        thread::spawn(move ||{
            println!("hello,{v:?}");
        }).join().unwrap();
    }




}