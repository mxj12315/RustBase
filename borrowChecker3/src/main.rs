/* 
 * impl接口的生命周期
 */
struct ImportantExcerpt<'a> {
    part: &'a str, // 'a 声明了str的生命周期
}

impl<'a> ImportantExcerpt<'a> {
    // 规则一只有一个参数则参数的返回值以参数的什么周期一致，所以i32不需要标注‘a的生命周期
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // 因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期
    fn announce_and_return_part(&self, announcement: &str) -> &str { 
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

