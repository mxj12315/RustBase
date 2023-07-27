fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    //let result = longest(string1.as_str(), string2);
    //println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        //函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致，
        //虽然方法上使用‘a 标记两个参数和返回值的生命周期一致，但是
        //string2生命周期较sting1短，所以整个longest1函数的生命周期和string2一样
        result = longest1(string1.as_str(), string2.as_str()); // Err:borrowed value does not live long enough,借用的变量生命周期不够久
    }
    println!("The longest string is {}", result);
}
/*
 *  longtest中的两个参数是&修饰符修饰的引用类型，返回值也是同样的point类型
 *  编译器并不知道两个函数的生命周期长短，导致方法报错
 */
/* fn longest(x: &str, y: &str) -> &str { // 报错： expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

/*  函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致
 *  使用‘ 符号来标注生命周期
 *  &i32        // 引用
 *  &'a i32     // 带有显式生命周期的引用
 *  &'a mut i32 // 带有显式生命周期的可变引用
 */
 fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str { // 报错： expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 
