use std::{sync::mpsc, thread, time::Duration};

/* 
    发送者发送多个数据
 */
fn main() {
    let (tx, rx) = mpsc::channel();
    let t1 = thread::spawn(move ||{
       let v = vec![1,2,3,4,5];

       for vitem in v {
            tx.send(vitem).unwrap();
            thread::sleep(Duration::from_secs(1));
       } ;
    });

    for received in rx{
        println!("{}", received);
    }


}