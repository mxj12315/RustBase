use std::{error::Error, fs, env};

//定义一个结构来整合用户输入的数据
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Self, &'static str> {
        // 对参数进行判断
        if args.len() < 3 {
            return Err("参数长度不符合");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case
        })
    }
}

// 解析参数
// fn parse_config(args:Vec<String>) ->Config{
//      //  Config::build(args)
// }

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    //  let content = fs::read_to_string(cfg.file_path)
    // .expect("打开文件出错了");

    // 改进，将异常处理抛出
    let contents: String = fs::read_to_string(cfg.file_path)?;
    let result: Vec<&str> = if cfg.ignore_case{
        search_case_insensitive(&cfg.query, &contents)
    }else{
        search(&cfg.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}
// 搜索函数
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            v.push(line);
        }
    }
    v
}
// 搜索函数-不区分大小写
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) ->Vec<&'a str>{
    let mut v = Vec::new();
    let query_lowercase = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lowercase) {
            v.push(line);
        }
    }
    v
}

// 单元测试
#[cfg(test)]
mod test {
    use super::*;
    #[test]

    // 区分大小写
    fn case_sensitive() {
        let query = "duct";
        // 注意双引号之后的反斜杠，这告诉 Rust 不要在字符串字面值内容的开头加入换行符
        let contents: &str = "\
        Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    // 不区分大小写
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}
