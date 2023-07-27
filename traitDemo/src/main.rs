use traitDemo::{NewsArticle,Tweet};
use traitDemo::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let new_article: NewsArticle = NewsArticle {
        headline:String::from("This is HeadLine"),
        author:String::from("This is author mxj"),
        location:String::from("location"),
        content:String::from("这是一段内容")
    };

    let result1: String = new_article.summarize1();// 调用子类的重写方法
    println!("result1={}",result1);

    let result2: String =  tweet.summarize1(); // 调用trait的默认实现
    println!("result2={}",result2);


}
