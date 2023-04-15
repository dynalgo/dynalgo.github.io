use super::color::Color;

#[derive(Copy, Clone)]
pub enum Tag {
    Created(Color),
    Selected(Color),
    Deleted(Color),
}
