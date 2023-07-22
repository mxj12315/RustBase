/*
    引用和借用
*/
fn main(){
    let s1 = String::from("hello");
    let ref_s1 = &s1; // s1的地址引用,称为借用，可以使用不可以对其进行修改
    // 使用借用的数据，正常执行
    let len = calculate_length(ref_s1);
    println!("len={}",len);

    // 修改没用的数据，编译报错
    // let new_str = change_str(ref_s1);
    // println!("new_str={:?}",new_str);

    // 添加mut后
    let mut s2 = String::from("hello");
    let ref_s2 = &mut s2; // s2的地址引用,称为借用，可以使用不可以对其进行修改
    change_str1(ref_s2);
    println!("ref_s2={}",ref_s2);
    println!("s2={}",s2);

    //  两个变量同时指向一个借用地址
    //  数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
    //      两个或更多指针同时访问同一数据。
    //      至少有一个指针被用来写入数据。
    //      没有同步数据访问的机制。s
    let mut s3 = String::from("hello");
    //let ref_s3 = &mut s3; // s3的地址引用,称为借用，可以使用不可以对其进行修改
    //let ref_s4 = &mut s3; // s3的地址引用,称为借用，可以使用不可以对其进行修改
    //println!("ref_s3={},ref_s4={}",ref_s3,ref_s4);// 编译报错，同时存在同一地址的两个可变引用

    // 不能在特定作用域中同时存在可变和不可变引用
    let mut s4 = String::from("hello");
    let ref_s5 = &s4; // 仅仅借用地址，不修改
    let ref_s6 = &s4; // 仅仅借用地址，不修改
    // let ref_s7 = &mut s4; // 借用地址，但是
    // println!("ref_s5={},ref_s6={},ref_s7={}",ref_s5,ref_s6,ref_s7); // println!不能同时引用可变和不可变的引用

    // 修改代码：先对不可变引用进行使用，再对可变引用进行使用
    println!("ref_s5={},ref_s6={}",ref_s5,ref_s6); // 先使用不可以变引用
    let ref_s8 = &mut s4; // 此时借用地址，可以修改
    println!("ref_s8={}",ref_s8);

}

/*
    获取字符串长度
*/
fn calculate_length(str:&String) -> usize{
    str.len()
}

/*
    尝试修改字符串,编译报错:
*/
// fn change_str(str: &String)->String{
//     str.push_str(",world"); // error[E0596]: cannot borrow immutable borrowed content `*str` as mutable
// }


/*
    尝试修改可变引用字符串:
*/
fn change_str1(str: &mut String){
    str.push_str(",world");
}

