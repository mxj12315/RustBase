/*
    weak弱指针的使用，子节点依赖于父节点(强指针)，但是父节点不依赖于子节点(弱指针)
 */
use std::{cell::RefCell, rc::{Weak, Rc}};

#[derive(Debug)]
struct Node{
    value: i32,
    parent:RefCell<Weak<Node>>,// 指向父级的弱引用
    chirldren: RefCell<Vec<Rc<Node>>>,   
}

fn main() {
    // 树叶
    let leaf: Rc<Node> = Rc::new(Node{
        value: 3,
        parent:RefCell::new(Weak::new()), //leaf 开始时没有父节点，所以我们新建了一个空的 Weak 引用实例。
        chirldren: RefCell::new(vec![])
    });
    // upgrade()尝试将 `Weak` 指针升级为“Rc”，如果成功则延迟删除内部值,否则返回 `None`
    println!("leaf 的父级 = {:?}", leaf.parent.borrow().upgrade());

    // 树干，此时branch的children是指向leaf的引用
    let branch: Rc<Node> = Rc::new(Node{
        value: 5,
        parent:RefCell::new(Weak::new()),
        chirldren: RefCell::new(vec![Rc::clone(&leaf)])
    });

    // downgrade()创建一个指向此分配的新[Weak]指针, 会将 weak_count 加 1
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // 将leaf的parent指向branch的弱引用
    // upgrade()尝试将 `Weak` 指针升级为“Rc”，如果成功则延迟删除内部值,否则返回 `None`
    println!("leaf 的父级 = {:?}", leaf.parent.borrow().upgrade());


}