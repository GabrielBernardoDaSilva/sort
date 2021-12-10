use crate::sort::SortAlgorithms;

#[derive(Clone, Copy, Debug)]
pub struct Bar {
    pub id: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
    pub color: ggez::graphics::Color,
}

impl Bar {
    pub fn new(id: i32, pos_x: f32, pos_y: f32, height: f32, color: ggez::graphics::Color) -> Self {
        Self {
            id,
            pos_x,
            pos_y,
            height,
            width: 29.0,
            color,
        }
    }
}

impl PartialOrd for Bar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&(*other).id)
    }
}

impl PartialEq for Bar {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl SortAlgorithms for Bar {
    fn get_key(&self) -> i32 {
        self.id
    }
}
