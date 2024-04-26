// 大乐透
// 5个红球 1-35
// 2个篮球 1-12
pub mod grand_lotto {
    use std::io;
    use chrono::Local;
    use crate::service::generate::rand as iRand;
    use crate::service::generate::hash as iHash;
    use crate::write_run_log;

    pub fn frame() {
        println!("欢迎来到大乐透~~~请选择你的幸运模式：");
        println!("1. 我即是天命");
        println!("2. 今天必中");
        println!("============================================================");

        let mut select_mod_input = String::new();
        io::stdin().read_line(&mut select_mod_input).expect("选择模式失败");

        let mut red: Vec<i32>;
        let mut blue: Vec<i32>;

        // 注意：输入的字符串带了\n，所以要trim
        match select_mod_input.as_str().trim() {
            "1" => {
                (red, blue) = select_rand();
            }
            "2" => {
                (red, blue) = select_hash();
            }
            _ => {
                println!("没有诚心！");
                return;
            }
        }

        if red.len() < 5 || blue.len() < 2 {
            println!("今天不宜！！！");
            return;
        }

        red.sort();
        blue.sort();
        println!("本次的运算结果是：");
        println!("红色球：{:?}", red);
        println!("蓝色球：{:?}", blue);

        write_run_log(&select_mod_input.trim(), red, blue).expect("写入日志失败");
    }


    pub fn select_rand() -> (Vec<i32>, Vec<i32>) {
        let mut red_boo: Vec<i32> = Vec::new();
        loop {
            iRand::rand_number_to_vec(&mut red_boo, 1, 35);
            if red_boo.len() == 5 {
                break;
            }
        }

        let mut blue_boo: Vec<i32> = Vec::new();
        loop {
            iRand::rand_number_to_vec(&mut blue_boo, 1, 12);
            if blue_boo.len() == 2 {
                break;
            }
        }

        red_boo.sort();
        blue_boo.sort();

        (red_boo, blue_boo)
    }

    /**
     * 用日期做hash得出固定数
     */
    pub fn select_hash() -> (Vec<i32>, Vec<i32>) {
        let date = Local::now().date_naive();
        println!("{}，今天的幸运数字是：", date);

        let candidate = iHash::hash_string_sha512(date.format("%Y-%m-%d").to_string().as_str());

        let mut red: Vec<i32> = Vec::new();
        let mut blue: Vec<i32> = Vec::new();
        for item in &candidate {
            if blue.len() > 1 {
                break;
            }

            if red.len() < 6 && item >= &1 && item <= &35 {
                if red.contains(&(*item as i32)) {
                    continue;
                }
                red.push(*item as i32);
            } else if item >= &1 && item <= &12 {
                if blue.contains(&(*item as i32)) {
                    continue;
                }
                blue.push(*item as i32);
            }
        }

        return (red, blue);
    }
}

// 双色球
pub mod union_lotto {
    use std::io;

    use chrono::Local;
    use rand::Rng;

    use crate::service::generate::hash as iHash;
    use crate::service::generate::rand as iRand;
    use crate::write_run_log;

    pub fn frame() {
        println!("欢迎来到双色球~~~请选择你的幸运模式：");
        println!("1. 我即是天命");
        println!("2. 今天必中");
        println!("============================================================");

        let mut select_mod_input = String::new();
        io::stdin().read_line(&mut select_mod_input).expect("选择模式失败");

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
                println!("没有诚心！");
                return;
            }
        }

        if red.len() < 6 || blue == 0 {
            println!("今天不宜！！！");
            return;
        }

        red.sort();
        println!("本次的运算结果是：");
        println!("红色球：{:?}", red);
        println!("蓝色球：{}", blue);

        write_run_log(&select_mod_input.trim(), red, blue).expect("写入日志失败");
    }

    pub fn use_rand() -> (Vec<i32>, i32) {

        // 红色球
        let mut red_vec: Vec<i32> = Vec::new();

        loop {
            iRand::rand_number_to_vec(&mut red_vec, 1, 33);

            // 需要6个球
            if red_vec.len() == 6 {
                break;
            }
        }

        red_vec.sort();

        // 蓝色球
        let blue_num: i32 = rand::thread_rng().gen_range(1..=16);

        return (red_vec, blue_num);
    }

    /**
     * 用日期做hash得出固定数
     */
    pub fn date_hash() -> (Vec<i32>, i32) {
        let date = Local::now().date_naive();
        println!("{}，今天的幸运数字是：", date);

        let candidate = iHash::hash_string_sha512(date.format("%Y-%m-%d").to_string().as_str());

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
}