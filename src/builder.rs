use macos::View;

pub trait Builder<T: View> {
    fn build(self) -> T;
}

impl<T: View> Builder<T> for T {
    fn build(self) -> T {
        self
    }
}
