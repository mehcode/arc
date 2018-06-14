extern crate square;

use square::*;

fn main() {
    let app = Application::new();

    let mut window = Window::new(800., 500.);
    window.set_title("Playground");

    let mut top = View::new();
    top.set_background_color(0x64B5F6);
    top.set_height(50.);

    let mut middle_left = View::new();
    middle_left.set_background_color(0x4CAF50);
    middle_left.set_width(50.);

    let mut middle_right = View::new();
    middle_right.set_background_color(0xEC407A);
    middle_right.set_width(50.);

    let mut middle_center = View::new();
    middle_center.set_background_color(0xFAFAFA);
    middle_center.set_flex_grow(1.);

    let mut middle = View::new();
    middle.set_flex_grow(1.);
    middle.set_flex_direction(FlexDirection::Row);
    middle.add_child(middle_left);
    middle.add_child(middle_center);
    middle.add_child(middle_right);

    let mut bottom = View::new();
    bottom.set_background_color(0xB0BEC5);
    bottom.set_height(50.);

    let mut root = View::new();
    root.add_child(top);
    root.add_child(middle);
    root.add_child(bottom);

    window.set_view(root);

    app.add_window(window);
    app.run();
}
