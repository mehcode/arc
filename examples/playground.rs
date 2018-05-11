extern crate arc;

use arc::*;

fn main() {
    let app = Application::new();

    let window = Window::new(900., 700.);
    window.set_title("Arc – Playground");

    let mut top_bar = SolidColor::new();
    top_bar.set_height(30.);
    top_bar.set_background_color(0xeaeaeb);

    let mut top_bar_border = SolidColor::new();
    top_bar_border.set_height(1.);
    top_bar_border.set_background_color(0xdbdbdc);

    let mut bottom_bar = SolidColor::new();
    bottom_bar.set_height(30.);
    bottom_bar.set_background_color(0xeaeaeb);

    let mut bottom_bar_border = SolidColor::new();
    bottom_bar_border.set_height(1.);
    bottom_bar_border.set_background_color(0xdbdbdc);

    let mut main = SolidColor::new();
    main.set_background_color(0xfafafa);

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
