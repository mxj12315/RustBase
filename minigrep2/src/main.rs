// 二进制项目的关注分离
//      将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
//      当命令行解析逻辑比较小时，可以保留在 main.rs 中。
//      当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

// 改造版
use std::env;
use std::process;
use minigrep1::Config;
use minigrep1::run;

fn main() {
    // 读取参数的逻辑
    let cfg: Config = Config::build(env::args()).unwrap_or_else(|error|{
        eprintln!("错误：{}",error);
        process::exit(1);
    }
    );
    println!("query={},file_path={}",cfg.query,cfg.file_path);
    // 打开文件的业务逻辑
    if let Err(e) = run(cfg)  {
        eprintln!("文件打开错误「{}」",e);
        process::exit(1);
    }


}
  
// 数组排序函数




