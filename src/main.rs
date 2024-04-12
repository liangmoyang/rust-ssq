use std::io;

use rand::Rng;

fn main() {
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
    println!("红色球：{:?}", red_vec);

    // 蓝色球
    let blue_num: i32 = rng.gen_range(1..=16);
    println!("蓝色球：{:?}", blue_num);

    println!("输入回车关闭");
    let mut input_key = String::new();
    io::stdin().read_line(&mut input_key).expect("Error");
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
