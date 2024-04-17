use chrono::Local;
use digest::Digest;
use sha2::Sha512;
/**
 * 用日期做hash得出固定数
 */
pub fn date_hash() -> (Vec<i32>, i32) {
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