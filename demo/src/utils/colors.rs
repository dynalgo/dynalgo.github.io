/// Color palette generator
use crate::utils::random::Random;

pub struct Colors {}

/// Returns a list of RGB type colors
impl Colors {
    pub fn colors(count: usize) -> Vec<(u8, u8, u8)> {
        assert!(count > 0 && count < 255 * 255 * 255);
        let mut colors = Vec::new();
        let levels_count = (count as f64).cbrt().ceil() as u8;
        let step = 255 / levels_count;
        for r in 1..=levels_count {
            for g in 1..=levels_count {
                for b in 1..=levels_count {
                    colors.push((r * step, g * step, b * step));
                }
            }
        }
        for _ in 1..colors.len() {
            let idx = Random::poor_random((colors.len() - 1) as u32);
            colors.swap(0, (idx + 1) as usize);
        }
        colors
    }
}
