use add_one;
use rand; //虽然在add_one.rs中引入了rand，但是在这里不可以使用，需要在外层公共的cargo.toml中添加
fn main() {
    let result = add_one::add_one(5);
    println!("{}",result);
}
