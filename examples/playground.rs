extern crate arc;

use arc::*;

fn main() {
    let app = Application::new();

    let window = Window::new(900., 700.);
    window.set_title("Arc – Playground");

    let mut top_bar = SolidColor::new(0xeaeaeb);
    top_bar.set_height(30.);

    let mut top_bar_border = SolidColor::new(0xdbdbdc);
    top_bar_border.set_height(1.);

    let mut bottom_bar = SolidColor::new(0xeaeaeb);
    bottom_bar.set_height(30.);

    let mut bottom_bar_border = SolidColor::new(0xdbdbdc);
    bottom_bar_border.set_height(1.);

    let main = SolidColor::new(0xfafafa);

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
