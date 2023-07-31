// 迭代器 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。
fn main() {
    let vec = vec![1, 2, 3];
    let mut iter = vec.iter(); // 该行在不消费迭代器时候不会有什么作用

    //使用迭代器进行消费  next方法被称为消费适配器
    println!("{:?}", iter.next()); // some(1)
    println!("{:?}", iter.next()); // some(2)
    println!("{:?}", iter.next()); // some(3)
    println!("{:?}", iter.next()); // None

    let vec1 = vec![1, 2, 3];
    let iter1 = vec1.iter(); // 获得可变的迭代器
    let sum: i32 = iter1.sum(); // 获取总和
    println!("{:?}", sum);

    let v1: Vec<i32> = vec![1, 2, 3];
    //迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。
    let v2 = v1.iter().map(|x| x + 1); // 这里只是map函数创建了一个新的迭代器
    println!("{:?}", v2);
    let v3: Vec<i32> = v2.collect();
    println!("{:?}", v3); //[2, 3, 4]
}
