#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}
/*
pub struct Params {
    pub duration_add: u32,
    pub duration_delete: u32,
    pub duration_move: u32,
    pub duration_select: u32,
    pub duration_color: u32,
    pub color_tag_created: Color,
    pub color_tag_selected: Color,
    pub color_tag_deleted: Color,
    pub color_node_fill: Color,
    pub color_node_border: Color,
    pub color_link_border: Color,
    pub color_text: Color,
    pub display_node_label: bool,
    pub display_node_value: bool,
    pub display_link_label: bool,
    pub display_link_value: bool,
    pub stroke_width_node: u32,
    pub stroke_width_link: u32,
    pub radius_node: u32,
}

impl Params {
    pub fn new() -> Params {
        Params {
            duration_add: 1000,
            duration_delete: 1000,
            duration_move: 1000,
            duration_select: 1000,
            duration_color: 1000,
            color_tag_created: Color::new(0, 0, 255),
            color_tag_selected: Color::new(191, 255, 0),
            color_tag_deleted: Color::new(255, 0, 0),
            color_node_fill: Color::new(255, 255, 255),
            color_node_border: Color::new(128, 139, 150),
            color_link_border: Color::new(128, 139, 150),
            color_text: Color::new(0, 0, 0),
            display_node_label: true,
            display_node_value: false,
            display_link_label: false,
            display_link_value: true,
            stroke_width_node: 2,
            stroke_width_link: 2,
            radius_node: 20,
        }
    }
}
*/
