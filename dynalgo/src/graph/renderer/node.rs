use super::color::Color;
use super::point::Point;
use super::tag::Tag;

#[derive(Copy, Clone)]
pub struct Node {
    id: u32,
    name: char,
    center: Point,
    center_freezed: bool,
    pub radius: u32,
    pub fill_color: Color,
    pub stroke_color: Color,
    pub text_color: Color,
    pub stroke_width: u32,
    tag: Option<Tag>,
}

impl Node {
    pub fn new(
        id: u32,
        name: char,
        center: Option<Point>,
        radius: u32,
        fill_color: Color,
        stroke_color: Color,
        text_color: Color,
        stroke_width: u32,
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

    pub fn move_to(&mut self, center: Point) {
        self.center = center;
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
