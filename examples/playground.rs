extern crate square;

use square::*;

fn main() {
    let app = Application::new();

    let window = Window::new(900., 700.);
    window.set_title("Square – Playground");

    let mut top_bar = SolidColor::new(0xeaeaeb);
    top_bar.set_height(30.);

    let mut top_bar_border = SolidColor::new(0xdbdbdc);
    top_bar_border.set_height(1.);

    let mut bottom_bar = SolidColor::new(0xeaeaeb);
    bottom_bar.set_height(30.);

    let mut bottom_bar_border = SolidColor::new(0xdbdbdc);
    bottom_bar_border.set_height(1.);

    let mut line_1 = Text::new();
    line_1.set_text_color(0x383a42);
    line_1.set_text("Hello World!");

    let mut main = Row::new();
    main.set_background_color(0xfafafa);
    main.add(line_1);

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
