use std::{error::Error, fs, env};

//定义一个结构来整合用户输入的数据
/// The `Config` struct represents a configuration for a search operation, including
/// the query, file path, and whether to ignore case.
/// 
/// Properties:
/// 
/// * `query`: A string that represents the search query or keyword.
/// * `file_path`: The `file_path` property is a string that represents the path to a
/// file. It is used to specify the location of the file that will be searched or
/// processed.
/// * `ignore_case`: A boolean value indicating whether the search should be
/// case-insensitive or not.
pub struct Config {
    pub query: String,// 需要搜索的字符串
    pub file_path: String,//文件路径
    pub ignore_case: bool,// 是否忽略大小写
}

impl Config {
/// The `build` function in Rust takes a vector of strings as arguments, checks if the length is at
/// least 3, and returns a struct with the query, file path, and a flag indicating whether to ignore
/// case.
/// 
/// Arguments:
/// 
/// * `args`: A vector of strings that contains the command line arguments passed to the program.
/// 
/// Returns:
/// 
/// The function `build` returns a `Result<Self, &'static str>`.
    pub fn build<T>(mut args:T) -> Result<Config, &'static str>
    where T: Iterator<Item = String>
     {
        args.next(); // 跳过第一个参数
        // 对参数进行判断
        let query = match args.next(){
            Some(s) => s,
            None => return Err("没有查询字符串"),
        };
        
        let file_path = match args.next(){
            Some(s) => s,
            None => return Err("没有文件路径"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            query,
            file_path,
            ignore_case
        })
    }
}

// 解析参数
// fn parse_config(args:Vec<String>) ->Config{
//      //  Config::build(args)
// }

/// The `run` function reads the contents of a file specified in the `cfg` struct, performs a search
/// based on the query string, and prints the matching lines.
/// 
/// Arguments:
/// 
/// * `cfg`: A struct that contains the configuration for the program. It includes the following fields:
/// 
/// Returns:
/// 
/// The function `run` is returning a `Result` type with the `Ok` variant containing `()` (an empty
/// tuple) if the function executes successfully, and the `Err` variant containing a `Box<dyn Error>` if
/// an error occurs.
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
 /// The `case_insensitive` function tests the `search_case_insensitive` 
 /// function by asserting that it
 /// returns the expected results for a given query and contents.
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
