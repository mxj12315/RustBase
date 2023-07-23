/*
       处理可恢复的错误
*/
use std::fs::File;
use std::io::Write;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    let mut file = 
    match f {
        Ok(file) => file, // 如果文件存在，返回文件句柄
        Err(error) =>
        // 如果文件不存在，创建文件
        {
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error), // 其他错误，直接panic
            }
        }
    };
    file.write_all(b"Hello, world!").expect("write failed");
    
}
