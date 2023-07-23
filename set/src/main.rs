/*
    set集合
 */
use std::collections::HashSet;
fn main() {
    let mut st = HashSet::new();
    st.insert("a");
    st.insert("b");
    st.insert("c");
    println!("st={:?}",st);

    // 遍历
    for item in st.iter() {
        println!("item={:?}",item);
    }

    // 删除元素
    st.remove("a");
    println!("st={:?}",st);

    // 判断是否包含某个元素
    let res = st.contains("b");
    println!("res={:?}",res);

    // 使用union方法合并两个集合
    let mut st1 = HashSet::new();
    st1.insert("a");
    st1.insert("b");
    st1.insert("c");
    let mut st2 = HashSet::new();
    st2.insert("a");
    st2.insert("b");
    st2.insert("d");
    let st3 = st1.union(&st2);
    println!("st3={:?}",st3); // {"a", "b", "c", "d"}

    // 使用difference方法获取两个集合的差集
    let st4 = st1.difference(&st2);
    println!("st4={:?}",st4); // {"c"}
}
