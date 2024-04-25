// 大乐透
pub mod grand_lotto {}

// 双色球
pub mod union_lotto {
    use chrono::Local;
    use rand::Rng;
    use crate::service::generate::hash;

    pub fn use_rand() -> (Vec<i32>, i32) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        // 红色球
        let mut red_vec: Vec<i32> = Vec::new();

        loop {
            crate::service::generate::rand::rand_number_to_vec(&mut red_vec, 1, 33);

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

    /**
     * 用日期做hash得出固定数
     */
    pub fn date_hash() -> (Vec<i32>, i32) {
        let date = Local::now().date_naive();
        println!("{}，今天的幸运数字是：", date);

        let candidate = hash::hash_string_sha512(date.format("%Y-%m-%d").to_string().as_str());

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