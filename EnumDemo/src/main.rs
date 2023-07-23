/*
    枚举类型
 */
// IP地址类型,不指定类型
#[derive(Debug)]
enum IPAddressType1 {
    IPv4,
    IPv6
}
#[derive(Debug)]
// Ip地址，指定枚举成员的类型使用()
enum IPAddressType2 {
    IPv4(u8,u8,u8,u8), // 指定类型 u8无符号8bit
    IPv6(String)
}
/*
为枚举实现方法
 */
enum Message{
    Write(String)
}
// 为Message枚举定义方法
impl Message {
    fn call(&self) {
        print!("Message枚举call方法")
    }
}


/*
    路由
 */
#[derive(Debug)]
struct Route1 {
    kind:IPAddressType1, // 类型
    address :String
}

#[derive(Debug)]
struct Route2{
    kind:IPAddressType2, // 类型
    address :String
}

fn main(){
    let route1 = Route1{
        kind:IPAddressType1::IPv4,
        address:String::from("192.168.0.1")
    };
    let route2 = Route1{
        kind:IPAddressType1::IPv6,
        address:String::from("::1")
    };
    dbg!(route1);
    dbg!(route2);

    // 使用指定类型的枚举值
    let route3 = Route2 {
        kind:IPAddressType2::IPv4(192, 168, 0, 13),
        address:String::from("::1")
    };
    dbg!(route3);


    // 调用枚举方法
    let mes = Message::Write(String::from("Hello Rust"));
    mes.call();
}