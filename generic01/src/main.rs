/*
    使用泛型改造函数
 */
use std::cmp;
 fn largest<T:PartialOrd >(list: &[T]) -> &T
  {
    let mut max = &list[0];
    for i in list {
        if *i > *max {
            max = i;
        }
    }
    max
}

fn largest1<T:PartialOrd+ Copy> ()->(){

}
fn largest2<T> ()->()
where PartialOrd+ Copy
{
    
}

enum  Res<T,E>{
    Ok(T),
    Err(E)
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    // 方法可以有自己泛型定义
    fn  drow<N,M>(self,a:M) {
        
    }
}


fn main() {
    let ve = vec![1,10,12,199];
    let n = largest(&ve);
    println!("n={:?}",n);

    let ve1 = vec!['a','c','e'];
    let m = largest(&ve1);
    println!("m={:?}",m);

    let res = Res{
        Ok:12,
        Err:"sss"
    };

}
