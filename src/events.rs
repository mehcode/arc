
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
    pub location: (f32, f32),
}
