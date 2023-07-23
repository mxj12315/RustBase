// RUST_BACKTRACE=1 cargo run 命令来打印出栈信息跟踪
fn main() {
    let ve = vec![1,2,3];
    let res = ve[5];
    println!("res={:?}",res);
}
