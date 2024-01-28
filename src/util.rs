use chrono::Local;
use rand::{distributions::Alphanumeric, Rng};

pub fn generate_string_id() -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

pub fn get_cur_date() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
