use std::{
    fs::OpenOptions,
    io::{self, Write},
    process,
};

use chrono::prelude::*;

mod service;

use service::mode::union_lotto;
use crate::service::mode::grand_lotto;

fn main() {
    io::stdout().flush().expect("Flush Fail"); // 刷新

    println!("您要玩什么？");
    println!("1. 大乐透");
    println!("2. 双色球");
    println!("============================================================");
    let mut select_method_input = String::new();
    io::stdin().read_line(&mut select_method_input).expect("选择玩法失败");

    match select_method_input.as_str().trim() {
        "1" => {
            grand_lotto::frame();
        }
        "2" => {
            union_lotto::frame();
        }
        _ => {
            println!("不玩拉倒！！！回车键退出");
            io::stdin().read_line(&mut String::new()).expect("退出失败");
            process::exit(0);
        }
    }

    println!("回车键退出");
    io::stdin().read_line(&mut String::new()).expect("退出");
}

// 记录日志到文本
fn write_run_log<T: std::fmt::Debug, V: std::fmt::Debug>(select_mod: &str, red: T, blue: V) -> Result<(), std::io::Error> {
    let content = format!(
        "{} 运算结果: \t选择模式【{}】 红色球{:?} 蓝色球[{:?}]\n",
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
