use super::Point;

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug)]
pub struct MouseDown {
    pub button: MouseButton,
    pub location: Point,
}

#[derive(Debug)]
pub struct MouseUp {
    pub button: MouseButton,
    pub location: Point,
}

#[derive(Debug)]
pub struct MouseEnter {
    pub location: Point,
}

#[derive(Debug)]
pub struct MouseLeave {
    pub location: Point,
}
