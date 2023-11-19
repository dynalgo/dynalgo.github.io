use std::time::{SystemTime, UNIX_EPOCH};

pub struct Random {}

impl Random {
    /// Returns a value between zero and max (excluded)
    pub fn poor_random(max: u32) -> u32 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        nanos % max
    }
}
