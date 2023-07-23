/*
    数组的可变引用和不可变引用
        报错原因：
            1.vector的可变引用v.push(4)修改了vector的长度，导致之前的不可变引用失效
            2.不可变引用item在v.push(4)之前创建，而v.push(4)之后，vector的长度发生了变化，导致之前的不可变引用失效
 */
fn main() {
    let mut v = vec![1, 2, 3];
    let mut item = &v[0]; // 使用数组的不可变引用
    //v.push(4); // 编译报错，可变引用，因为数组的可变引用已经被创建，不允许修改数组
    println!("item={}", item);
}
