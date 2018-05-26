extern crate square;

use square::*;

fn main() {
    let app = Application::new();
    let window = Window::new(600., 300.);
    window.set_title("Hello World");

    let mut view = Row::new();

    let mut hello = Text::new();
    hello.set_text("Hello");
    hello.set_text_color(0x545454);

    let mut world = Text::new();
    world.set_text("World");
    world.set_text_color(0x545454);

    view.add(hello);
    view.add(world);

    window.set_content_view(view);
    window.activate();

    app.activate();
    app.run();
}
