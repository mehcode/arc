extern crate square;

use square::*;

fn main() {
    let app = Application::new();

    let mut window = Window::new(400., 400.);
    window.set_title("Square — Layout");

    let mut top_left = View::new();
    top_left.set_background_color(0xe7484d);
    top_left.set_flex_grow(1.0);

    let mut top_right = View::new();
    top_right.set_background_color(0xfdf52c);
    top_right.set_width_percent(0.3);
    top_right.set_margin(Edge::Start, 20.);

    let mut top = View::new();
    top.set_flex_direction(FlexDirection::Row);
    top.set_height_percent(0.3);
    top.add_child(top_left);
    top.add_child(top_right);

    let mut bottom_left = View::new();
    bottom_left.set_background_color(0xe7484d);
    bottom_left.set_flex_grow(1.0);

    let mut bottom_right_1 = View::new();
    bottom_right_1.set_background_color(0x3199fa);
    bottom_right_1.set_flex_grow(1.0);

    let mut bottom_right_2 = View::new();
    bottom_right_2.set_margin(Edge::Top, 20.);
    bottom_right_2.set_background_color(0x3199fa);
    bottom_right_2.set_flex_grow(1.0);

    let mut bottom_right = View::new();
    bottom_right.set_margin(Edge::Start, 20.);
    bottom_right.set_flex_grow(1.0);
    bottom_right.add_child(bottom_right_1);
    bottom_right.add_child(bottom_right_2);

    let mut bottom = View::new();
    bottom.set_margin(Edge::Top, 20.);
    bottom.set_flex_direction(FlexDirection::Row);
    bottom.set_flex_grow(1.);
    bottom.add_child(bottom_left);
    bottom.add_child(bottom_right);

    let mut root = View::new();
    root.set_padding(Edge::All, 20.);
    root.set_background_color(0x0d2a4e);
    root.add_child(top);
    root.add_child(bottom);

    window.set_view(root);

    app.add_window(window);
    app.run();
}
