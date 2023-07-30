/* 
 *  获取命令行参数
 *      cargo run -- 参数1 参数2
 */
use std::env;
fn main() {
    let args:Vec<String> = env::args().collect();
    let file_name = &args[0];// args的切片 第一个参数二进制文件的名称
    let query = &args[1];
    let file_path = &args[1];
    println!("file_name={},\nquery={},\nfile_path={}",file_name,query,file_path);
}
