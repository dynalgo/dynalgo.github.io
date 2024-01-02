pub struct Color;

impl Color {
    pub fn colors() -> Vec<(u8, u8, u8)> {
        vec![
            (255, 0, 0),
            (255, 127, 0),
            (255, 255, 0),
            (0, 255, 0),
            (0, 0, 255),
            (46, 43, 95),
            (139, 0, 255),
        ]
    }

    pub fn default() -> (u8, u8, u8) {
        (47, 79, 79)
    }

    pub fn disabled() -> (u8, u8, u8) {
        (192, 192, 192)
    }
}
