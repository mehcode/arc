use super::Point;

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone)]
pub struct MouseDown {
    pub button: MouseButton,
    pub location: Point,
}

#[derive(Debug, Clone)]
pub struct MouseUp {
    pub button: MouseButton,
    pub location: Point,
}

#[derive(Debug, Clone)]
pub struct MouseEnter {
    pub location: Point,
}

#[derive(Debug, Clone)]
pub struct MouseLeave {
    pub location: Point,
}
