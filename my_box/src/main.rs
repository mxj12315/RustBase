/* 
 * Box<T>智能指针
 *  指针存放在栈上，数据存放在堆上
 *  实现了 Deref trait，它允许 Box<T> 值被当作引用对待，离开作用域时候自动释放
 *  适用于递归数据链表
 */

/* 
enum  List{ //ecursive type `List` has infinite size. List具有无限大小
    Cons(i32,List),
    Nil,
} 
*/
enum  List{ //ecursive type `List` has infinite size. List具有无限大小
    Cons(i32,Box<List>),
    Nil,
} 
//  需要足够储存两个 值的空间，依此类推。
// 因为 Message 值所需的空间等于储存其最大成员的空间大小
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn main() {
    let a = List::Cons(5,Box::new(List::Cons(12, Box::new(List::Nil))));
}
