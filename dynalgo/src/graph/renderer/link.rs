use super::color::Color;
use super::point::Point;
use super::tag::Tag;

#[derive(Copy, Clone)]
pub struct Link {
    id: u32,
    from: char,
    to: char,
    from_center: Point,
    to_center: Point,
    bidirect: bool,
    value: u8,
    stroke_color: Color,
    stroke_color_init: Color,
    text_color: Color,
    stroke_width: u8,
    tag: Option<Tag>,
}

impl Link {
    pub fn new(
        id: u32,
        from: char,
        to: char,
        from_center: Point,
        to_center: Point,
        bidirect: bool,
        value: u8,
        stroke_color: Color,
        text_color: Color,
        stroke_width: u8,
    ) -> Link {
        Link {
            id,
            from,
            to,
            from_center,
            to_center,
            bidirect,
            value,
            stroke_color,
            stroke_color_init: stroke_color,
            text_color,
            stroke_width,
            tag: None,
        }
    }

    pub fn id(&self) -> String {
        format!("{}{}{}", self.from, self.to, self.id)
    }

    pub fn from(&self) -> char {
        self.from
    }

    pub fn to(&self) -> char {
        self.to
    }

    pub fn from_center(&self) -> &Point {
        &self.from_center
    }

    pub fn to_center(&self) -> &Point {
        &self.to_center
    }

    pub fn update_from_center(&mut self, center: Point) {
        self.from_center = center;
    }

    pub fn update_to_center(&mut self, center: Point) {
        self.to_center = center;
    }

    pub fn bidirect(&self) -> bool {
        self.bidirect
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn stroke_color(&self) -> Color {
        self.stroke_color
    }

    pub fn set_stroke_color(&mut self, color: Color) {
        self.stroke_color = color
    }

    pub fn stroke_color_init(&self) -> Color {
        self.stroke_color_init
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color
    }

    pub fn text_color(&self) -> Color {
        self.text_color
    }

    pub fn stroke_width(&self) -> u8 {
        self.stroke_width
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
}
