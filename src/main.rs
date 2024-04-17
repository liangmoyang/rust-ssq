use std::{
    fs::OpenOptions,
    io::{self, Write},
    process,
};

use chrono::prelude::*;
use digest::Digest;
use rand::Rng;
use sha2::Sha512;

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
            (red, blue) = use_rand();
        }
        "2" => {
            (red, blue) = date_hash();
        }
        _ => {
            println!("没有诚心！回车键退出");
            io::stdin().read_line(&mut String::new()).expect("退出");
            process::exit(0);
        }
    }

    if red.len() < 6 || blue == 0 {
        println!("今天不宜！！！回车键退出");
        process::exit(0);
    }

    red.sort();
    println!("红色球：{:?}", red);
    println!("蓝色球：{}", blue);

    write_run_log(&select_mod_input.trim(), red, blue).expect("写入日志失败");

    println!("回车键退出");
    io::stdin().read_line(&mut String::new()).expect("退出");
}

/**
 * 纯随机数的方式
 */
fn use_rand() -> (Vec<i32>, i32) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    // 红色球
    let mut red_vec: Vec<i32> = Vec::new();

    loop {
        rand_number(&mut red_vec);

        // 需要6个球
        if red_vec.len() == 6 {
            break;
        }
    }

    red_vec.sort();

    // 蓝色球
    let blue_num: i32 = rng.gen_range(1..=16);

    return (red_vec, blue_num);
}

fn rand_number(vec1: &mut Vec<i32>) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let random_number: i32 = rng.gen_range(1..=33);

    // 去重
    if vec1.contains(&random_number) {
        rand_number(vec1);
    } else {
        vec1.push(random_number);
    }
}

// ======================================================================================================

/**
 * 用日期做hash得出固定数
 */
fn date_hash() -> (Vec<i32>, i32) {
    let date = Local::now().date_naive();
    println!("{}，今天的幸运数字是：", date);

    let candidate = hash_string_sha512(date.format("%Y-%m-%d").to_string().as_str());

    let mut red: Vec<i32> = Vec::new();
    let mut blue: i32 = 0;
    for item in &candidate {
        if blue != 0 {
            break;
        }

        if red.len() < 6 && item > &0 && item < &34 {
            if red.contains(&(*item as i32)) {
                continue;
            }
            red.push(*item as i32);
        } else if item > &0 && item < &17 {
            blue = *item as i32;
        }
    }

    return (red, blue);
}

fn hash_string_sha512(s: &str) -> Vec<u8> {
    let mut hasher = Sha512::new();

    hasher.update(s.as_bytes());
    hasher.finalize().to_vec()
}

// ******************************************************************************************************

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
