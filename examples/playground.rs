extern crate arc;

use arc::*;

fn main() {
    let app = Application::new();

    let window = Window::new(900., 700.);
    window.set_title("Arc – Playground");

    let top_bar = SolidColor::builder(0xeaeaeb).height(30.);
    let top_bar_border = SolidColor::builder(0xdbdbdc).height(1.);

    let bottom_bar = SolidColor::builder(0xeaeaeb).height(30.);
    let bottom_bar_border = SolidColor::builder(0xdbdbdc).height(1.);

    let main = SolidColor::builder(0xfafafa);

    let mut container = Column::new();
    container.add(top_bar);
    container.add(top_bar_border);
    container.add(main);
    container.add(bottom_bar_border);
    container.add(bottom_bar);

    window.set_content_view(container);
    window.activate();

    app.activate();
    app.run();
}
