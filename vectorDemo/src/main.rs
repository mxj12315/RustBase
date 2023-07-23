/*
  vector可变数组
 */

fn main() {
    // 使用宏创建
    let mut ve1: Vec<i32> = vec![1,2,3];
    println!("ve1= {:#?}",ve1);
    println!("ve1[0]={:?}",ve1[0]);
    println!("ve1[0]={:?}",&ve1[0]); // 使用数组的首地址
    // 使用some来访问
    let res: Option<&i32> =  ve1.get(1);
    println!("ve1[1]={:?}",res);
    match res {
        Some(x)=> println!("x={}",x),
        None => println!("未匹配到")
    }

    for  i in 0..ve1.len() 
    {
        ve1[i] += 2;
    }

    for j in [1..2] 
    {
        print!("j={}",j);
    }
    println!("修改后的ve1= {:#?}",ve1);


    // 使用new
    let mut ve2 = Vec::new();
    ve2.push("v1");
    ve2.push("v2");
    ve2.push("v3");
    println!("v2={:#?}",ve2);
    ve2.remove(0);
    println!("remove后的ve2={:?}",ve2);

 
   
}
