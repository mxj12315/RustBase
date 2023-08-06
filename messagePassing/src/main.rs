/*
message passing消息传递
mpsc多个生产者，单个消费者（multiple producer, single consumer）的缩写

信道：信道是一个通用编程概念，表示数据从一个线程发送到另一个线程。
 */
use std::{sync::mpsc, thread};
fn main() {
    // channel返回一个元组，第一个元素是发送端，第二个元素是接收端
    let (se, re) = mpsc::channel(); // 创建一个信道
    let t1: thread::JoinHandle<()> = thread::spawn(move || {
        let str: String = String::from("hello");
        se.send(str).unwrap();  
        // val在上一步中已经发生了左右权限的转移
        // println!("val is {}", val);
    });

    // 使用接收端接收数据
    let res: String = re.recv().unwrap();
    println!("res = {}", res);
    t1.join().unwrap();


}
