/*
    match匹配
 */
enum Color{
    Red,
    Bluee,
    Black,
    Other(String)
}

fn main(){
    let mat1: u32 = match_test(Color::Black);
    let mat2: u32 = match_test(Color::Bluee);
    let mat3: u32 = match_test(Color::Other(String::from("Rust")));
    let mat4: u32 = match_test(Color::Red);
    println!("mat1={}",mat1);
    println!("mat2={}",mat2);
    println!("mat3={}",mat3);
    println!("mat4={}",mat4);


    println!();
    // 通用匹配_和绑定操作符@
    let num: i32 = 7;
    match num {
        1 => println!("1"),
        2..=3 => println!("2"),
        n @ 7 => {
            println!("n={}",n)
        },
        _ => println!("no match")
    }
}

fn match_test(col:Color) -> u32{
    let a:u32;
    match col{
        Color::Red=> a = 1,
        Color::Bluee=> a= 2,
        Color::Black => a = 3,
        Color::Other(v) =>{
            println!("v={}",v);
            a =4
        } 
    };
    a
}