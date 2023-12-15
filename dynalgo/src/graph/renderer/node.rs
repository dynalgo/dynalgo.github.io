use super::color::Color;
use super::point::Point;
use super::tag::Tag;

#[derive(Copy, Clone)]
pub struct Node {
    id: u32,
    name: char,
    center: Point,
    center_freezed: bool,
    radius: u8,
    fill_color: Color,
    stroke_color: Color,
    stroke_color_init: Color,
    text_color: Color,
    stroke_width: u8,
    tag: Option<Tag>,
}

impl Node {
    pub fn new(
        id: u32,
        name: char,
        center: Option<Point>,
        radius: u8,
        fill_color: Color,
        stroke_color: Color,
        text_color: Color,
        stroke_width: u8,
    ) -> Node {
        let center_freezed = center.is_some();
        let center = match center {
            Some(c) => c,
            None => Point::new(0, 0),
        };
        Node {
            id,
            name,
            center,
            center_freezed,
            radius,
            fill_color,
            stroke_color,
            stroke_color_init: stroke_color,
            text_color,
            stroke_width,
            tag: None,
        }
    }

    pub fn id(&self) -> String {
        format!("{}{}", self.name, self.id)
    }

    pub fn name(&self) -> char {
        self.name
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn center_freeze(&mut self, freezed: bool) {
        self.center_freezed = freezed;
    }

    pub fn center_freezed(&self) -> bool {
        self.center_freezed
    }

    pub fn radius(&self) -> u8 {
        self.radius
    }

    pub fn move_to(&mut self, center: Point) {
        self.center = center;
    }

    pub fn tag(&mut self, tag: Option<Tag>) {
        self.tag = tag;
    }

    pub fn tag_created(&self) -> bool {
        match self.tag {
            Some(Tag::Created) => true,
            _ => false,
        }
    }

    pub fn tag_deleted(&self) -> bool {
        match self.tag {
            Some(Tag::Deleted) => true,
            _ => false,
        }
    }

    pub fn stroke_width(&self) -> u8 {
        self.stroke_width
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color
    }

    pub fn fill_color(&self) -> Color {
        self.fill_color
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color
    }

    pub fn text_color(&self) -> Color {
        self.text_color
    }

    pub fn stroke_color(&self) -> Color {
        self.stroke_color
    }

    pub fn stroke_color_init(&self) -> Color {
        self.stroke_color_init
    }

    pub fn set_stroke_color(&mut self, color: Color) {
        self.stroke_color = color
    }
}
