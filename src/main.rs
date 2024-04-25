use std::{
    fs::OpenOptions,
    io::{self, Write},
    process,
};

use chrono::prelude::*;

mod service;

use service::mode::union_lotto;

fn main() {

    let mut select_mod_input = String::new();

    io::stdout().flush().expect("Flush Fail"); // 刷新

    println!("请选择你的幸运模式：");
    println!("1. 纯天命");
    println!("2. 今天必中");
    println!("============================================================");

    io::stdin().read_line(&mut select_mod_input).expect("获取输入失败");

    let mut red: Vec<i32>;
    let mut blue: i32 = 0;

    // 注意：输入的字符串带了\n，所以要trim
    match select_mod_input.as_str().trim() {
        "1" => {
            (red, blue) =  union_lotto::use_rand();
        }
        "2" => {
            (red, blue) = union_lotto::date_hash();
        }
        _ => {
            println!("没有诚心！回车键退出");
            io::stdin().read_line(&mut String::new()).expect("退出");
            process::exit(0);
        }
    }

    if red.len() < 6 || blue == 0 {
        println!("今天不宜！！！回车键退出");
        io::stdin().read_line(&mut String::new()).expect("退出");
        process::exit(0);
    }

    red.sort();
    println!("红色球：{:?}", red);
    println!("蓝色球：{}", blue);

    write_run_log(&select_mod_input.trim(), red, blue).expect("写入日志失败");

    println!("回车键退出");
    io::stdin().read_line(&mut String::new()).expect("退出");
}

// 记录日志到文本
fn write_run_log(select_mod: &str, red: Vec<i32>, blue: i32) -> Result<(), std::io::Error> {
    let content = format!(
        "{} 运算结果: \t选择模式【{}】 红色球{:?} 蓝色球[{}]\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        select_mod,
        red,
        blue
    );

    let mut obj: OpenOptions = OpenOptions::new();
    let options = obj.write(true).create(true).append(true);

    let mut file = options.open("log.txt")?;

    file.write_all(content.as_bytes())?;

    file.sync_all()?;

    Ok(())
}
