use std::fmt::Write;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 参数switch的不同该方法会返回两种类型，目前回报错，需要后续章节内容
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

// 泛型限制
pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify2<T:Summary>(item1: T, item2: T) -> T { item1 }


// 多个trait限制的三种方式
pub fn notify3(item1: &(impl Summary + Copy), item2: &impl Summary) {}
pub fn notify4<T:Summary + Copy>(item1: T, item2: T) -> T { item1 }
pub fn notify5<T,E>(item1: T, item2: T) -> T 
where 
    T:Copy + PartialOrd + Summary,
    E:Write, // 最后这里有一个逗号
{ item1 }

