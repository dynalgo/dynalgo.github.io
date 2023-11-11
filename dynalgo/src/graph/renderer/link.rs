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
    pub stroke_color: Color,
    pub text_color: Color,
    pub stroke_width: u32,
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
        stroke_width: u32,
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

    pub fn tag(&mut self, tag: Option<Tag>) {
        self.tag = tag;
    }

    pub fn tag_created(&self) -> bool {
        match self.tag {
            Some(Tag::Created(_)) => true,
            _ => false,
        }
    }

    pub fn tag_selected(&self) -> bool {
        match self.tag {
            Some(Tag::Selected(_)) => true,
            _ => false,
        }
    }

    pub fn tag_deleted(&self) -> bool {
        match self.tag {
            Some(Tag::Deleted(_)) => true,
            _ => false,
        }
    }

    pub fn stroke_color_tagged(&self) -> Color {
        match self.tag {
            Some(tag) => match tag {
                Tag::Created(color) => color,
                Tag::Selected(color) => color,
                Tag::Deleted(color) => color,
            },
            None => self.stroke_color,
        }
    }
}
