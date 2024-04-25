// 随机数
pub mod rand {
    use rand::Rng;

    pub fn rand_number_to_vec(r: &mut Vec<i32>, range_left: i32, range_right: i32) {
        let mut rng = rand::thread_rng();

        let num = rng.gen_range(range_left..=range_right);

        // 重复则递归
        if r.contains(&num) {
            rand_number_to_vec(r, range_left, range_right);
        } else {
            r.push(num);
        }
    }
}

// 哈希
pub mod hash {
    use digest::Digest;
    use sha2::Sha512;

    pub fn hash_string_sha512(s: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();

        hasher.update(s.as_bytes());
        hasher.finalize().to_vec()
    }
}